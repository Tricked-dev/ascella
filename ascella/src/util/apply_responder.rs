// use paste::paste;
#[macro_export]
macro_rules! apply_responders {
   ($($name:ident),*) => {
use paste::paste;
        paste! {
            $(
                impl Responder for $name {
                    type Body = BoxBody;

                    fn respond_to(self, _req: &HttpRequest) -> actix_web::HttpResponse {
                        let body = serde_json::to_string(&self).unwrap();

                        actix_web::HttpResponse::Ok()
                        .content_type("application/json")
                        .body(body)
                }
              }
            )*
        }
    }
}
