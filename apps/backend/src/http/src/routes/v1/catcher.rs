use crate::send_message;
use salvo::prelude::*;
use salvo::Catcher;

pub struct Handle404;
impl Catcher for Handle404 {
  fn catch(&self, _req: &Request, res: &mut Response) -> bool {
    println!("{}", _req.uri());

    if let Some(StatusCode::NOT_FOUND) = res.status_code() {
      res.render_json(&send_message(404, false, &format!("Could not find {}", _req.uri())));
      true
    } else {
      false
    }
  }
}
