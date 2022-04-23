use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{take_ss, ScreenshotKind};
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_ext::prelude::*;
use clipboard_ext::x11_bin::ClipboardContext as LinuxContext;
use home::home_dir;
use native_dialog::{MessageDialog, MessageType};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AscellaConfig {
  #[serde(rename = "authorization")]
  pub auth: Option<String>,
  pub headers: Option<String>,
  pub command: Option<String>,
}

pub fn update_config<T: Into<PathBuf>>(path: T) -> Result<(), Error> {
  let path: PathBuf = path.into();
  let r: Value = std::fs::read_to_string(&path)
    .map(|r| serde_json::from_str(&r))
    .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?
    .map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;

  let config: AscellaConfig =
    serde_json::from_str(&serde_json::to_string(&r["Headers"]).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;

  let mut write_path = home_dir().unwrap();

  write_path.extend(&[".ascella", "config.toml"]);
  std::fs::write(&write_path, toml::to_string_pretty(&config).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?).map_err(|x| Error::new(ErrorKind::Other, x.to_string()))?;
  Ok(())
}

pub fn screenshot(t: ScreenshotKind) -> iced::Result {
  let mut write_path = home_dir().unwrap();
  write_path.extend(&[".ascella", "config.toml"]);

  let config: AscellaConfig = if let Ok(config_raw) = std::fs::read_to_string(write_path) {
    if let Ok(config) = toml::from_str(&config_raw) {
      config
    } else {
      println!("Your config is invalid please use a valid ascella config");
      MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("invalid config")
        .set_text("Your config is invalid please use a valid ascella config")
        .show_alert()
        .unwrap();
      return Ok(());
    }
  } else {
    println!("config not detected please upload your config");
    MessageDialog::new()
      .set_type(MessageType::Info)
      .set_title("config not detected please upload your config")
      .set_text("config not detected please upload your config\n\nPlease add a config file you can do this using the gui")
      .show_alert()
      .unwrap();
    return Ok(());
  };

  let mut path = home_dir().unwrap();

  path.extend(&[".ascella", "images"]);
  std::fs::create_dir_all(&path).unwrap();
  let filename = chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S.png").to_string();
  path.extend(&[&filename]);
  if let Some(command) = config.command {
    let replaced = command.replace("%image", &path.clone().into_os_string().into_string().unwrap());
    let mut parts = replaced.trim().split_whitespace();

    let command = parts.next().unwrap();

    let args = parts;

    Command::new(command).args(args).spawn().unwrap();
  } else {
    take_ss(t, path.clone().into_os_string().into_string().unwrap(), true);
  }
  upload(path).unwrap();
  Ok(())
}

pub fn upload<P: AsRef<Path>>(path: P) -> Result<String, Error> {
  let mut write_path = home_dir().unwrap();
  write_path.extend(&[".ascella", "config.toml"]);

  let config_raw = if let Ok(config_raw) = std::fs::read_to_string(write_path) {
    config_raw
  } else {
    println!("config not detected please upload your config");
    MessageDialog::new()
      .set_type(MessageType::Info)
      .set_title("config not detected please upload your config")
      .set_text("config not detected please upload your config\n\nPlease add a config file you can do this using the gui")
      .show_alert()
      .unwrap();
    return Ok("".to_owned());
  };

  let form = multipart::Form::new().file("photo", path).unwrap();

  let mut headers = HeaderMap::new();

  let config: AscellaConfig = if let Ok(config) = toml::from_str(&config_raw) {
    config
  } else {
    println!("Your config is invalid please use a valid ascella config");
    MessageDialog::new()
      .set_type(MessageType::Info)
      .set_title("invalid config")
      .set_text("Your config is invalid please use a valid ascella config")
      .show_alert()
      .unwrap();
    return Ok("".to_owned());
  };

  headers.insert("authorization", HeaderValue::from_str(&config.auth.unwrap()).unwrap());
  headers.insert("user-agent", HeaderValue::from_str("Ascella-uploader").unwrap());

  let client = reqwest::Client::new();
  let mut resp = client.post("https://ascella.wtf/v2/ascella/upload").headers(headers).multipart(form).send().unwrap();

  let text = resp.text().unwrap();

  let r: Value = serde_json::from_str(&text).unwrap();
  let url = r["url"].as_str().expect("Invalid image type");

  println!("{url}");
  copy(url.clone().to_owned());

  Ok(url.to_owned())
}

fn copy(t: String) {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

  ctx.set_contents(t.to_owned()).unwrap();

  // Linux workarounds
  #[cfg(target_os = "linux")]
  {
    use std::io::prelude::*;
    use std::process::Stdio;
    use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

    let ctx = LinuxContext::new().ok();
    if let Some(mut ctx) = ctx {
      ctx.set_contents(t.clone()).ok();
    }

    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    if let Ok((mut pipe, _)) = result {
      let mut contents = vec![];
      pipe.read_to_end(&mut contents).expect("Failed to read pipe");
    }

    let child = Command::new("xclip").arg("-selection").arg("clipboard").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn();
    if let Ok(mut child) = child {
      {
        let child_stdin = child.stdin.as_mut();
        if let Some(child_stdin) = child_stdin {
          child_stdin.write_all((&t).to_string().as_bytes()).ok();
        }
      }
      let _ = child.wait().ok();
    }

    Command::new("wl-copy").arg(&t).spawn().ok();
  }
}
