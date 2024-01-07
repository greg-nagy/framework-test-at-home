use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

use std::time::Duration;

#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DB_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/portal_dev".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route(
            "/count",
            get(count)
        )
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// we can extract the connection pool with `State`
async fn count(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar::<_, i32>("SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .map(|count| count.to_string())
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
