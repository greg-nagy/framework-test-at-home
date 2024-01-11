use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};


use std::{env, net::{SocketAddr, Ipv4Addr}, io};
use tokio::net::{TcpListener, TcpSocket};

mod database;

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
             std::process::exit(0);
        },
    };

    let pg_connection = PgConnection::connect(db_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello prefork :3002!" }))
        .route("/count", get(count))
        .with_state(pg_connection);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3002));
    let listener = reuse_listener(addr).expect("couldn't bind to addr");

    println!("Started axum server at 3002");

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
