use std::{
    any::Any,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Result;
use rusqlite::Connection;
use tokio::sync::{mpsc, oneshot};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub(crate) type DbOutput = Box<dyn Any + Send + Sync + 'static>;
pub(crate) type DbInteraction =
    Box<dyn FnOnce(&mut InnerDatabase) -> Result<DbOutput> + Send + Sync + 'static>;

pub(crate) struct DbInstruction {
    pub interaction: DbInteraction,
    pub output: oneshot::Sender<Result<DbOutput>>,
}

pub struct Database {
    sender: mpsc::Sender<DbInstruction>,
}

pub(crate) struct InnerDatabase {
    pub connection: rusqlite::Connection,
    last_optimize: Instant,
}

impl Database {
    pub async fn open_in_memory() -> Result<Self> {
        let connection = tokio::task::spawn_blocking(|| Connection::open_in_memory()).await??;
        Self::open(connection).await
    }

    pub async fn open_file<P: AsRef<Path> + Send + Sync + 'static>(path: P) -> Result<Self> {
        let connection = tokio::task::spawn_blocking(|| Connection::open(path)).await??;
        Self::open(connection).await
    }

    pub(crate) async fn call<O: Send + Sync + 'static>(
        &self,
        func: impl FnOnce(&mut InnerDatabase) -> Result<O> + Send + Sync + 'static,
    ) -> Result<O> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(DbInstruction {
                interaction: Box::new(|db| func(db).map(|x| Box::new(x) as DbOutput)),
                output: sender,
            })
            .await
            .map_err(|_| anyhow::anyhow!("database is gone"))?;
        receiver
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("database disappeared during call")))
            .and_then(|x| {
                x.downcast()
                    .map(|x| *x)
                    .map_err(|_| anyhow::anyhow!("mismatched output type for call"))
            })
    }

    pub async fn open(conn: Connection) -> Result<Self> {
        let (sender, receiver) = mpsc::channel(16);
        std::thread::spawn(move || {
            InnerDatabase {
                connection: conn,
                last_optimize: Instant::now(),
            }
            .inner_thread(receiver)
        });
        let self_ = Database { sender };
        self_
            .call(|db| {
                embedded::migrations::runner().run(&mut db.connection)?;
                Ok(())
            })
            .await?;

        Ok(self_)
    }
}

impl InnerDatabase {
    fn inner_thread(mut self, mut receiver: mpsc::Receiver<DbInstruction>) {
        while let Some(next) = receiver.blocking_recv() {
            let output = (next.interaction)(&mut self);
            let _ = next.output.send(output);
        }
    }

    pub fn optimize(&mut self) -> Result<()> {
        if self.last_optimize.elapsed() < Duration::from_secs(60 * 15) {
            return Ok(());
        }
        self.connection.execute(r"PRAGMA OPTIMIZE;", [])?;
        self.last_optimize = Instant::now();
        Ok(())
    }
}
