use axum::Json;
use error::Result;
use serde::Serialize;
use serde_json::{json, Value};

pub mod auth_route;
pub mod error;
pub mod user_route;

fn response_ok<T: Serialize>(data: T) -> Result<Json<Value>> {
    Ok(Json(json!({
        "status": "success",
        "code": 200,
        "data": data
    })))
}
