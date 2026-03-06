use serde::{Deserialize, Serialize};
use ts_rs::TS;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]

pub struct Template {
  pub id: i32,
  pub image_path: String,
  pub qr_x: String,
  pub qr_y: String,
  pub num_x: String,
  pub num_y: String,
  pub num_width: String,
  pub num_height: String,
  pub num_padding: String,
  pub num_border_radius: String,
  pub num_border_color: String,
  pub num_border_width: String,
  pub num_border_style: String, 
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateTemplate {
  pub image_path: String,
  pub qr_x: String,
  pub qr_y: String,
  pub num_x: String,
  pub num_y: String,
  pub num_width: String,
  pub num_height: String,
  pub num_padding: String,
  pub num_border_radius: String,
  pub num_border_color: String,
  pub num_border_width: String,
  pub num_border_style: String, 
}