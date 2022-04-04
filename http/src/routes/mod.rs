// pub mod v1;
pub mod v2;
pub mod prelude {
  pub use crate::{create_config, ran_str, random_emojis, send_message, send_text_webhook, upload_success, validate_request, Error};
  pub use actix_multipart::Multipart;
  pub use actix_web::{HttpRequest, HttpResponse};
  pub use anyhow::Result;
  pub use ascella_database::queries::*;
  pub use byte_unit::Byte;
  pub use futures::{StreamExt, TryStreamExt};
  pub use image::io::Reader as ImageReader;
  pub use paperclip::actix::api_v2_operation;
  pub use paperclip::actix::*;
  pub use serde::{Deserialize, Serialize};
  pub use serde_json::{from_str, json, Value};
  pub use std::{
    collections::HashMap,
    fs::{metadata, read},
    io::{Cursor, Write},
  };
  pub use tokio::fs::create_dir_all;
}
