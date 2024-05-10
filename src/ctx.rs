use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;

use crate::models::user::User;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub user: Option<User>,
    pub trace: Arc<Mutex<String>>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx {
            user: None,
            trace: Arc::new(Mutex::new(String::new())),
        }
    }

    pub async fn push_trace(&mut self, trace: &str) {
        let mut trace_lock = self.trace.lock().await;
        trace_lock.push_str(trace);
    }
}
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Infallible> {
        Ok(parts.extensions.get::<Ctx>().unwrap().clone())
    }
}
