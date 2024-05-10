use crate::ctx::Ctx;
use crate::repositories::user_repository;
use crate::routes::error::Error;
use crate::{models::user::UserForLogin, routes::error::Result};
use axum::{extract::State, routing::post, Json, Router};
use bcrypt::verify;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use serde_json::Value;
use sha2::Sha384;
use sqlx::PgPool;
use std::collections::BTreeMap;

use super::response_ok;

pub fn routes(db: PgPool) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .with_state(db)
}

async fn login(
    mut ctx: Ctx,
    State(db): State<PgPool>,
    Json(payload): Json<UserForLogin>,
) -> Result<Json<Value>> {
    ctx.push_trace(" -> login").await;

    let user = user_repository::get_user_by_id(&db, payload.id)
        .await
        .map_err(|_| Error::CredentialNotMatch)?;

    if !verify(payload.password, &user.password).map_err(|_| Error::CredentialNotMatch)? {
        return Err(Error::CredentialNotMatch);
    }

    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let user_id = user.id.to_string();

    let mut claims: BTreeMap<&str, &str> = BTreeMap::new();
    claims.insert("sub", &user_id);

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"sangat rahasia").unwrap();

    let token = Token::new(header, claims)
        .sign_with_key(&key)
        .map_err(|_| Error::CredentialNotMatch)?;

    response_ok(token.as_str())
}
