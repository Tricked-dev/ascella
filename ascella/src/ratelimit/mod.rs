use crate::util::Error as AscellaError;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::ResponseError;
use actix_web::{body::MessageBody, error, Error};
use futures::future;
use governor::{
  clock::{Clock, DefaultClock},
  state::keyed::DefaultKeyedStateStore,
  Quota, RateLimiter,
};

use std::{
  cell::RefCell,
  net::IpAddr,
  num::NonZeroU32,
  rc::Rc,
  sync::Arc,
  task::{Context, Poll},
  time::Duration,
};

const DEFAULT_PERIOD: Duration = Duration::from_millis(500);
const DEFAULT_BURST_SIZE: u32 = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GovernorConfigBuilder {
  period: Duration,
  burst_size: u32,
  methods: Option<Vec<Method>>,
}

impl Default for GovernorConfigBuilder {
  fn default() -> Self {
    GovernorConfigBuilder {
      period: DEFAULT_PERIOD,
      burst_size: DEFAULT_BURST_SIZE,
      methods: None,
    }
  }
}

impl GovernorConfigBuilder {
  pub fn period(&mut self, duration: Duration) -> &mut Self {
    self.period = duration;
    self
  }
  pub fn per_second(&mut self, seconds: u64) -> &mut Self {
    self.period = Duration::from_secs(seconds);
    self
  }

  pub fn per_millisecond(&mut self, milliseconds: u64) -> &mut Self {
    self.period = Duration::from_millis(milliseconds);
    self
  }

  pub fn per_nanosecond(&mut self, nanoseconds: u64) -> &mut Self {
    self.period = Duration::from_nanos(nanoseconds);
    self
  }

  pub fn burst_size(&mut self, burst_size: u32) -> &mut Self {
    self.burst_size = burst_size;
    self
  }

  pub fn methods(&mut self, methods: Vec<Method>) -> &mut Self {
    self.methods = Some(methods);
    self
  }

  pub fn finish(&mut self) -> Option<GovernorConfig> {
    if self.burst_size != 0 && self.period.as_nanos() != 0 {
      Some(GovernorConfig {
        limiter: Arc::new(RateLimiter::keyed(Quota::with_period(self.period).unwrap().allow_burst(NonZeroU32::new(self.burst_size).unwrap()))),
        methods: self.methods.clone(),
      })
    } else {
      None
    }
  }
}

#[derive(Clone, Debug)]
pub struct GovernorConfig {
  limiter: Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>,
  methods: Option<Vec<Method>>,
}

impl Default for GovernorConfig {
  fn default() -> Self {
    GovernorConfigBuilder {
      period: DEFAULT_PERIOD,
      burst_size: DEFAULT_BURST_SIZE,
      methods: None,
    }
    .finish()
    .unwrap()
  }
}

impl GovernorConfig {
  pub fn secure() -> Self {
    GovernorConfigBuilder {
      period: Duration::from_secs(4),
      burst_size: 2,
      methods: None,
    }
    .finish()
    .unwrap()
  }
}

pub struct Governor {
  limiter: Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>,
  methods: Option<Vec<Method>>,
}

impl Governor {
  pub fn new(config: &GovernorConfig) -> Governor {
    Governor {
      limiter: config.limiter.clone(),
      methods: config.methods.clone(),
    }
  }
}

impl<S, B> Transform<S, ServiceRequest> for Governor
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  B: MessageBody,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = GovernorMiddleware<S>;
  type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    future::ok(GovernorMiddleware::<S> {
      service: Rc::new(RefCell::new(service)),
      limiter: self.limiter.clone(),
      methods: self.methods.clone(),
    })
  }
}

pub struct GovernorMiddleware<S> {
  service: std::rc::Rc<std::cell::RefCell<S>>,
  limiter: Arc<RateLimiter<IpAddr, DefaultKeyedStateStore<IpAddr>, DefaultClock>>,
  methods: Option<Vec<Method>>,
}

impl<S, B> Service<ServiceRequest> for GovernorMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  B: MessageBody,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = future::Either<future::Ready<Result<ServiceResponse<B>, actix_web::Error>>, S::Future>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    if let Some(configured_methods) = &self.methods {
      if !configured_methods.contains(req.method()) {
        let fut = self.service.call(req);
        return future::Either::Right(fut);
      }
    }
    let ip: IpAddr = req
      .headers()
      .get("CF-Connecting-IP")
      .map(|r| r.to_str().unwrap().parse().unwrap_or_else(|_| "127.0.0.0".parse().unwrap()))
      .unwrap_or_else(|| if let Some(addr) = req.peer_addr() { addr.ip() } else { "127.0.0.0".parse().unwrap() });
    match self.limiter.check_key(&ip) {
      Ok(_) => {
        let fut = self.service.call(req);
        future::Either::Right(fut)
      }

      Err(negative) => {
        let wait_time = negative.wait_time_from(DefaultClock::default().now()).as_secs();
        log::info!("Rate limit exceeded for client-IP [{}], quota reset in {}s", &ip, &wait_time);
        let wait_time_str = wait_time.to_string();
        let body = format!("Too many requests, retry in {}s", &wait_time_str);
        // let response = actix_web::HttpResponse::TooManyRequests()
        //     .insert_header((actix_web::http::header::RETRY_AFTER, wait_time_str))
        //     .body(body.to_owned());
        let response = AscellaError::RateLimit { message: body.clone() };
        future::Either::Left(future::err(error::InternalError::from_response(body, response.error_response()).into()))
      }
    }
  }
}
