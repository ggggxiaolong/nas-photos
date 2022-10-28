use chrono::{DateTime, FixedOffset};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Object)]
pub struct Config {
    pub update_time: DateTime<FixedOffset>,
    pub scan_path: String,
    pub thumb_path: String,
    pub scan_time: Option<DateTime<FixedOffset>>,
}

#[derive(FromRow, Serialize, Deserialize, Object)]
pub struct Photo {
    id: i64,
    base_path: String,
    media_type: String,
    file_type: String,
    file_name: String,
    create_time: DateTime<FixedOffset>,
    hash_code: String,
    file_size: i64,
    favorite: bool,
    width: i32,
    height: i32,
    iso: i32,
    camera_model: String,
    camera_make: String,
    exposure_time: String,
    f_number: String,
    focal_length: String,
}
