use axum::body::Body;
use axum::http::{Method, Uri};
use axum::{middleware::map_response, response::Response, Router};
use ctx::Ctx;
use routes::auth_route;
use routes::user_route;
use serde_json::json;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, postgres::Postgres, Error, PgPool};
use std::sync::Arc;
use tokio::net::TcpListener;

mod ctx;
mod middleware;
mod models;
mod repositories;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = get_db_pool().await.unwrap();

    let routes = Router::new()
        .merge(user_route::routes(pool.clone()))
        .merge(auth_route::routes(pool.clone()))
        .layer(map_response(response_mapper))
        .layer(axum::middleware::from_fn_with_state(
            pool.clone(),
            middleware::auth_middleware::auth_resolver,
        ));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn response_mapper(mut ctx: Ctx, uri: Uri, req_method: Method, res: Response) -> Response {
    ctx.push_trace(" -> response_mapper").await;
    let trace_lock = ctx.trace.lock().await;

    let error = res
        .extensions()
        .get::<Arc<routes::error::Error>>()
        .map(Arc::as_ref);

    if let Some(error) = error {
        let (code, message) = error.to_client();
        let response_error = json!({
            "status": "error",
            "code": code.as_u16(),
            "message": message,
        });

        println!(
            "UserId: {:?}, URI: {:?}, Method: {:?}, Error: {:?}, Trace: {:?} , Response: {:?}",
            ctx.user.and_then(|u| Some(u.id)),
            uri,
            req_method,
            error,
            *trace_lock,
            response_error
        );

        return Response::new(Body::from(response_error.to_string()));
    }

    println!(
        "UserId: {:?}, URI: {:?}, Method: {:?}, Trace: {:?} , Response: {:?},",
        ctx.user.and_then(|u| Some(u.id)),
        uri,
        req_method,
        *trace_lock,
        res
    );

    res
}

async fn get_db_pool() -> Result<PgPool, Error> {
    let database_url = "postgres://postgres:postgres@localhost/sandbox";

    if !Postgres::database_exists(database_url).await? {
        Postgres::create_database(database_url).await?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
