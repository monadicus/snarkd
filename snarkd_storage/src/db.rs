use std::{
    ops::Deref,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Result;
use tokio::sync::Mutex;
use tokio_rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub struct Database {
    connection: Connection,
    last_optimize: Mutex<Instant>,
}

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.connection
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

        Ok(Database {
            connection: conn,
            last_optimize: Mutex::new(Instant::now()),
        })
    }

    pub async fn optimize(&self) -> Result<()> {
        let mut last_optimize = self.last_optimize.lock().await;
        if last_optimize.elapsed() < Duration::from_secs(60 * 15) {
            return Ok(());
        }
        self.call(|c| c.execute(r"PRAGMA OPTIMIZE;", [])).await?;
        *last_optimize = Instant::now();
        Ok(())
    }
}
