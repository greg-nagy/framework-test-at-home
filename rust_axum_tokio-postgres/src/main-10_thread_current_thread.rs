use std::env;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

mod database;

use self::database::{DatabaseConnection, PgConnection};

async fn count(DatabaseConnection(conn): DatabaseConnection) -> impl IntoResponse {
    let count = conn.get_count().await.expect("error loading world");

    (StatusCode::OK, count)
}

#[tokio::main]
async fn main() {
    let db_url = match env::var("DB_URL") {
        Ok(value) => value,
        Err(e) => {
            println!("Couldn't read MY_ENV_VAR ({})", e);
            return; // or handle the error as needed
        },
    };

    let pg_connection = PgConnection::connect(db_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/count", get(count))
        .with_state(pg_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
