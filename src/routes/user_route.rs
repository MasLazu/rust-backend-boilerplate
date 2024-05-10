use super::response_ok;
use crate::ctx::Ctx;
use crate::middleware::auth_middleware;
use crate::models::user::{Role, UserForCreate};
use crate::repositories::{self, user_repository};
use crate::routes::error::Error;
use crate::routes::error::Result;
use axum::extract::{Path, State};
use axum::middleware;
use axum::routing::{post, put};
use axum::{routing::get, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{json, Value};
use sqlx::PgPool;

pub fn routes(db: PgPool) -> Router {
    let admin = Router::new()
        .route("/users", post(create_user).delete(delete_user))
        .route_layer(middleware::from_fn(auth_middleware::admin_only));

    let authenticated = Router::new()
        .route("/users/id", put(update_user))
        .route_layer(middleware::from_fn(auth_middleware::authenticated_only));

    Router::new()
        .merge(admin)
        .merge(authenticated)
        .route("/users", get(get_all_users))
        .route("/users/:id", get(get_user_by_id))
        .with_state(db)
}

async fn get_all_users(mut ctx: Ctx, State(db): State<PgPool>) -> Result<Json<Value>> {
    ctx.push_trace(" -> get_all_users").await;

    let users = user_repository::get_all_users(&db).await;

    match users {
        Ok(users) => response_ok(users),
        Err(_) => Err(Error::DatabaseError),
    }
}

async fn get_user_by_id(
    mut ctx: Ctx,
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>> {
    ctx.push_trace(" -> get_user_by_id").await;

    let user = user_repository::get_user_by_id(&db, id).await;

    match user {
        Ok(user) => response_ok(user),
        Err(repositories::error::Error::RowNotFound) => Err(Error::NotFound),
        Err(_) => Err(Error::DatabaseError),
    }
}

async fn create_user(
    mut ctx: Ctx,
    State(db): State<PgPool>,
    Json(mut payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    ctx.push_trace(" -> create_user").await;

    payload.password = hash(payload.password, DEFAULT_COST).map_err(|_| Error::HashFail)?;

    let user = user_repository::insert_user(&db, payload.into()).await;

    match user {
        Ok(u) => response_ok(u),
        Err(repositories::error::Error::UniqueConstraintViolation) => Err(Error::IdAlreadyUsed),
        Err(_) => Err(Error::DatabaseError),
    }
}

async fn update_user(
    mut ctx: Ctx,
    State(db): State<PgPool>,
    Path(id): Path<i32>,
    Json(mut payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    ctx.push_trace(" -> update_user").await;

    let user = ctx.user.ok_or(Error::Unauthorized)?;

    if user.role != Role::Admin && user.id != id {
        return Err(Error::Unauthorized);
    }

    payload.password = hash(payload.password, DEFAULT_COST).map_err(|_| Error::HashFail)?;
    payload.id = id;

    let user = user_repository::update_user(&db, payload.into())
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn delete_user(
    mut ctx: Ctx,
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>> {
    ctx.push_trace(" -> delete_user").await;

    user_repository::delete_user(&db, id)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(json!({}))
}
