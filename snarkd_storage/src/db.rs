use std::{ops::Deref, path::Path};

use anyhow::Result;
use tokio_rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub struct Database(Connection);

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Database {
    pub async fn open_in_memory() -> Result<Self> {
        Self::open(Connection::open_in_memory().await?).await
    }

    pub async fn open_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::open(Connection::open(path).await?).await
    }

    pub async fn open(conn: Connection) -> Result<Self> {
        conn.call(|conn| embedded::migrations::runner().run(conn))
            .await?;

        Ok(Database(conn))
    }
}
