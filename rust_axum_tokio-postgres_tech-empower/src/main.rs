use std::env;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

mod database;
mod server;

use self::database::{DatabaseConnection, PgConnection};

async fn count(DatabaseConnection(conn): DatabaseConnection) -> impl IntoResponse {
    let count = conn.get_count().await.expect("error loading world");

    (StatusCode::OK, count)
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    //for _ in 1..num_cpus::get() {
    for _ in 1..10 {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(serve());
        });
    }
    rt.block_on(serve());
}

async fn serve() {
    let db_url = match env::var("DB_URL") {
        Ok(value) => value,
        Err(e) => {
            println!("Couldn't read DB_URL ({})", e);
            return; // or handle the error as needed
        },
    };    

    // setup connection pool
    let pg_connection = PgConnection::connect(db_url).await;

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/count", get(count))
        .with_state(pg_connection);

    server::builder()
        .serve(router.into_make_service())
        .await
        .unwrap();
}

