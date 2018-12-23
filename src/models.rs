#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use serde_json;

#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub password: String,
  pub email: String,
  pub created: NaiveDateTime,
  pub modified: NaiveDateTime,
  pub settings: Option<serde_json::Value>,
}
