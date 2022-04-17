use crate::http::models::report::ReportData;
use crate::prelude::*;

/// set domain
///
/// set the domain of the user
#[api_v2_operation(tags(Dashboard), consumes = "application/json", produces = "application/json")]
#[post("/report")]
pub async fn post(report_data: web::Json<ReportData>) -> Result<OkResponse<SendMessage>, Error> {
  let image = get_image::exec(report_data.id.to_string()).await;
  match image {
    Ok(image) => {
      send_text_webhook(format!(
        "**[REPORT][IMAGE]** [image](<https://ascella.wtf/v2/ascella/view/{image}.png>) **[OWNER]** ({id})",
        image = image.vanity,
        id = image.owner,
      ))
      .await
      .ok();
      Ok(OkResponse(SendMessage::new(200, true, "Reported")))
    }
    Err(_) => Ok(OkResponse(SendMessage::new(404, false, "Image not found"))),
  }
}
