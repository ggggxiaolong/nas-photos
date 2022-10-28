use poem_openapi::{Object};
use serde::{Deserialize, Serialize};

#[derive(Object, Deserialize)]
pub struct UpdateConfig{
    pub scan_path: String,
    pub thumb_path: String,
}