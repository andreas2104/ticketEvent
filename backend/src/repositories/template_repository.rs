use crate::db::DbPool;
use crate::domain::template::{Template, CreateTemplate};
use sqlx::{self};

pub struct TemplateRepository {
  pool: DbPool,
}

impl TemplateRepository {
  pub fn new(pool: DbPool) -> Self {
    self { pool }
  }
  pub async fn create(&self, templates:CreateTemplate) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO templates (image_path, qr_x, qr_y, num_x, num_y, num_width, num_height, num_padding, num_border_radius, num_border_color, num_border_width, num_border_style) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
    .bind(&templates.image_path)
    .bind(&templates.qr_x)
    .bind(&templates.qr_y)
    .bind(&templates.num_x)
    .bind(&templates.num_y)
    .bind(&templates.num_width)
    .bind(&templates.num_height)
    .bind(&templates.num_padding)
    .bind(&templates.num_border_radius)
    .bind(&templates.num_border_color)
    .bind(&templates.num_border_width)
    .bind(&templates.num_border_style)
    .execute(&self.pool)
    .await?;
    Ok(result.last_insert_rowid())
  }
  pub async fn find_all(&self) -> Result<Vec<Template>, sqlx::Error> {
    let templates = sqlx::query_as::<_, Template>("SELECT * FROM templates")
      .fetch_all(&self.pool)
      .await?;
    Ok(templates)
  }
}