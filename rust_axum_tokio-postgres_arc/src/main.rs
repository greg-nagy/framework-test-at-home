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
            println!("Couldn't read DB_URL ({})", e);
            return; // or handle the error as needed
        },
    };

    let pg_connection = PgConnection::connect(db_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello arc :3001!" }))
        .route("/count", get(count))
        .with_state(pg_connection);

    println!("Started axum server at 3001");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
