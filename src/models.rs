use chrono::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct BlogPost {
  pub id: i32,
  pub title: String,
  pub body: Option<String>,
  pub created_at: DateTime<Utc>,
  pub is_public: bool
}