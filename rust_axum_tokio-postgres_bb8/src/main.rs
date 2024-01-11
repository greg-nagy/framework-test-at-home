use std::env;

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Router,
};
use bb8::Pool;
use bb8_postgres::{PostgresConnectionManager, tokio_postgres::NoTls};

#[tokio::main]
async fn main() {
    let db_url = match env::var("DB_URL") {
        Ok(value) => value,
        Err(e) => {
            println!("Couldn't read DB_URL ({})", e);
            return; // or handle the error as needed
        },
    };    
    let manager =
        PostgresConnectionManager::new_from_stringlike(db_url, NoTls)
            .unwrap();
    let pool = Pool::builder()
        .max_size(30)
        .build(manager)
        .await
        .unwrap();

    // build our application with some routes
    let app = Router::new()
        .route( "/", get(|| async { "Hello world!" }))
        .route( "/count", get(fetch_count))
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn fetch_count(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let sql_query = "SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1";
    let row = conn
        .query_one(sql_query, &[])
        .await
        .map_err(internal_error)?;
    let count: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(count.to_string())
}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}