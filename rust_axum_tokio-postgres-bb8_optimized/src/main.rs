use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use std::{net::{SocketAddr, Ipv4Addr}, io};
use tokio::net::{TcpListener, TcpSocket};

mod database;
// mod server;

use self::database::{DatabaseConnection, PgConnection};

async fn count(DatabaseConnection(conn): DatabaseConnection) -> impl IntoResponse {
    let count = conn.get_count().await.expect("error loading world");

    (StatusCode::OK, count)
}

fn main() {
    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // //for _ in 1..num_cpus::get() {
    // for _ in 1..10 {
    //     std::thread::spawn(move || {
    //         let rt = tokio::runtime::Builder::new_current_thread()
    //             .enable_all()
    //             .build()
    //             .unwrap();
    //         rt.block_on(serve());
    //     });
    // }
    // rt.block_on(serve());

    let rt = tokio::runtime::Builder::new_multi_thread()
        // .worker_threads(10) // specify the number of threads here
        .enable_all()
        .build()
        .unwrap();    

    // for _ in 0..10 {
    //     rt.spawn(serve());
    // }

    rt.block_on(serve());

}

async fn serve() {
    let database_url: String = "postgresql://postgres:postgres@127.0.0.1:5432/portal_dev".to_string();

    // setup connection pool
    let pg_connection = PgConnection::connect(database_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/count", get(count))
        .with_state(pg_connection);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = reuse_listener(addr).expect("couldn't bind to addr");

    println!("Started axum server at 3000");

    axum::serve(listener, app).await.unwrap();
}

fn reuse_listener(addr: SocketAddr) -> io::Result<TcpListener> {
    let socket = match addr {
        SocketAddr::V4(_) => TcpSocket::new_v4()?,
        SocketAddr::V6(_) => TcpSocket::new_v6()?,
    };

    #[cfg(unix)]
    {
        if let Err(e) = socket.set_reuseport(true) {
            eprintln!("error setting SO_REUSEPORT: {e}");
        }
    }

    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;
    socket.listen(1024)
}