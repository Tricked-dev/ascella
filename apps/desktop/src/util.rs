use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::{take_ss, ScreenshotKind};
use clipboard::{ClipboardContext, ClipboardProvider};
use home::home_dir;
use native_dialog::{MessageDialog, MessageType};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AscellaConfig {
  #[serde(rename = "x-user-id")]
  pub id: Option<String>,
  #[serde(rename = "x-user-token")]
  pub token: Option<String>,
  #[serde(rename = "user-agent")]
  pub headers: Option<String>,
}

pub fn update_config<T: Into<PathBuf>>(path: T) -> Result<(), Error> {
  let path: PathBuf = path.into();
  let r: Value = std::fs::read_to_string(&path)
    .map(|r| serde_json::from_str(&r))
    .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?
    .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;

  let config: AscellaConfig = serde_json::from_str(&serde_json::to_string(&r["Headers"]).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?)
    .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;

  let mut write_path = home_dir().unwrap();

  write_path.extend(&[".ascella", "config.toml"]);
  std::fs::write(
    &write_path,
    toml::to_string_pretty(&config).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?,
  )
  .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;
  Ok(())
}

pub fn screenshot(t: ScreenshotKind) -> iced::Result {
  let mut path = home_dir().unwrap();

  path.extend(&[".ascella", "images"]);
  std::fs::create_dir_all(&path).unwrap();
  let filename = chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S.png").to_string();
  path.extend(&[&filename]);

  take_ss(t, path.clone().into_os_string().into_string().unwrap(), true);

  let form = multipart::Form::new().file("photo", path).unwrap();

  let mut headers = HeaderMap::new();
  let mut write_path = home_dir().unwrap();

  write_path.extend(&[".ascella", "config.toml"]);
  let config_raw = if let Ok(config_raw) = std::fs::read_to_string(write_path) {
    config_raw
  } else {
    MessageDialog::new()
      .set_type(MessageType::Info)
      .set_title("config not detected please upload your config")
      .set_text("config not detected please upload your config\n\nPlease add a config file you can do this using the gui")
      .show_alert()
      .unwrap();
    println!("config not detected please upload your config");
    return Ok(());
  };

  let config: AscellaConfig = if let Ok(config) = toml::from_str(&config_raw) {
    config
  } else {
    MessageDialog::new()
      .set_type(MessageType::Info)
      .set_title("invalid config")
      .set_text("Your config is invalid please use a valid ascella config")
      .show_alert()
      .unwrap();
    println!("Your config is invalid please use a valid ascella config");
    return Ok(());
  };
  headers.insert("x-user-id", HeaderValue::from_str(&config.id.unwrap()).unwrap());
  headers.insert("x-user-token", HeaderValue::from_str(&config.token.unwrap()).unwrap());
  headers.insert("user-agent", HeaderValue::from_str("Ascella-uploader").unwrap());

  let client = reqwest::Client::new();
  let mut resp = client
    .post("https://ascella.wtf/v2/ascella/upload")
    .headers(headers)
    .multipart(form)
    .send()
    .unwrap();

  let text = resp.text().unwrap();

  let r: Value = serde_json::from_str(&text).unwrap();
  let url = r["url"].as_str().unwrap();
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  ctx.set_contents(url.to_owned()).unwrap();
  Ok(())
}
