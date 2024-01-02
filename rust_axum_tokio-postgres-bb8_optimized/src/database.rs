use std::{convert::Infallible, io, sync::Arc};

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use futures::StreamExt;

use tokio::pin;
use tokio_postgres::{connect, Client, NoTls, Statement};


#[derive(Debug)]
pub enum PgError {
    Io(io::Error),
    Pg(tokio_postgres::Error),
}

impl From<io::Error> for PgError {
    fn from(err: io::Error) -> Self {
        PgError::Io(err)
    }
}

impl From<tokio_postgres::Error> for PgError {
    fn from(err: tokio_postgres::Error) -> Self {
        PgError::Pg(err)
    }
}

/// Postgres interface
pub struct PgConnection {
    client: Client,
    count: Statement,
}

impl PgConnection {
    pub async fn connect(db_url: String) -> Arc<PgConnection> {
        let (cl, conn) = connect(&db_url, NoTls)
            .await
            .expect("can not connect to postgresql");

        // Spawn connection
        tokio::spawn(async move {
            if let Err(error) = conn.await {
                eprintln!("Connection error: {error}");
            }
        });


        let query = "SELECT count FROM presence_counters WHERE name = 'group_sittings' ORDER BY updated_at DESC LIMIT 1";
        let count = cl.prepare(query).await.unwrap();

        Arc::new(PgConnection {
            client: cl,
            count,
        })
    }
}

impl PgConnection {
    pub async fn get_count(&self) -> Result<String, PgError> {
        let stream = self.client.query_raw::<_, _, &[i32; 0]>(&self.count, &[]).await?;
        pin!(stream);
        let row = stream.next().await.unwrap()?;
        let value: i32 = row.get(0);
        Ok(value.to_string())
    }
}

pub struct DatabaseConnection(pub Arc<PgConnection>);

#[async_trait]
impl FromRequestParts<Arc<PgConnection>> for DatabaseConnection {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        pg_connection: &Arc<PgConnection>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(pg_connection.clone()))
    }
}

