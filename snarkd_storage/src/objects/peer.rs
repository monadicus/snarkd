use std::net::SocketAddr;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::params;

use crate::Database;

#[derive(strum::IntoStaticStr, Clone, Copy, strum::EnumString)]
pub enum PeerDirection {
    /// peer connected to us
    Inbound,
    /// we connected to peer
    Outbound,
    /// we have never been connected to this peer
    Unknown,
}

#[derive(Clone, Copy)]
pub struct PeerData {
    pub address: SocketAddr,
    pub last_peer_direction: PeerDirection,
    pub block_height: u32,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub last_connected: Option<DateTime<Utc>>,
    pub blocks_synced_to: u64,
    pub blocks_synced_from: u64,
    pub blocks_received_from: u64,
    pub blocks_sent_to: u64,
    pub connection_fail_count: u64,
    pub connection_success_count: u64,
}

impl PeerData {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            last_peer_direction: PeerDirection::Unknown,
            block_height: 0,
            first_seen: None,
            last_seen: None,
            last_connected: None,
            blocks_synced_to: 0,
            blocks_synced_from: 0,
            blocks_received_from: 0,
            blocks_sent_to: 0,
            connection_fail_count: 0,
            connection_success_count: 0,
        }
    }

    pub fn merge_from(&mut self, from: &Self) {
        assert_eq!(self.address, from.address);
        self.last_peer_direction = from.last_peer_direction;
        self.block_height = from.block_height;
        self.first_seen = from.first_seen.or(self.first_seen);
        self.last_seen = from.last_seen.or(self.last_seen);
        self.last_connected = from.last_connected.or(self.last_connected);
        self.blocks_synced_to = from.blocks_synced_to.max(self.blocks_synced_to);
        self.blocks_synced_from = from.blocks_synced_from.max(self.blocks_synced_from);
        self.blocks_received_from = from.blocks_received_from.max(self.blocks_received_from);
        self.blocks_sent_to = from.blocks_sent_to.max(self.blocks_sent_to);
        self.connection_fail_count = from.connection_fail_count.max(self.connection_fail_count);
        self.connection_success_count = from
            .connection_success_count
            .max(self.connection_success_count);
    }

    pub async fn save(&self, db: &Database) -> Result<()> {
        let self_ = *self;
        db.call(move |db| {
            let mut stmt = db.prepare_cached(
                "
                INSERT INTO peers (
                    address,
                    last_peer_direction,
                    block_height,
                    first_seen,
                    last_seen,
                    last_connected,
                    blocks_synced_to,
                    blocks_synced_from,
                    blocks_received_from,
                    blocks_sent_to,
                    connection_fail_count,
                    connection_success_count
                )
                VALUES (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                )
                ON CONFLICT(address)
                DO UPDATE SET
                    last_peer_direction = excludede.last_peer_direction,
                    block_height = excluded.block_height,
                    first_seen = excluded.first_seen,
                    last_seen = excluded.last_seen,
                    last_connected = excluded.last_connected,
                    blocks_synced_to = excluded.blocks_synced_to,
                    blocks_synced_from = excluded.blocks_synced_from,
                    blocks_received_from = excluded.blocks_received_from,
                    blocks_sent_to = excluded.blocks_sent_to,
                    connection_fail_count = excluded.connection_fail_count,
                    connection_success_count = excluded.connection_success_count
            ",
            )?;

            stmt.execute(params![
                self_.address.to_string(),
                self_.block_height,
                self_.first_seen.map(|x| x.naive_utc().timestamp()),
                self_.last_seen.map(|x| x.naive_utc().timestamp()),
                self_.last_connected.map(|x| x.naive_utc().timestamp()),
                self_.blocks_synced_to,
                self_.blocks_synced_from,
                self_.blocks_received_from,
                self_.blocks_sent_to,
                self_.connection_fail_count,
                self_.connection_success_count,
            ])?;
            Ok(())
        })
        .await
    }

    pub async fn load_all(db: &Database) -> Result<Vec<Self>> {
        db.call(|db| {
            let mut stmt = db.prepare_cached(
                "
                SELECT
                    address,
                    last_peer_direction
                    block_height,
                    first_seen,
                    last_seen,
                    last_connected,
                    blocks_synced_to,
                    blocks_synced_from,
                    blocks_received_from,
                    blocks_sent_to,
                    connection_fail_count,
                    connection_success_count
                FROM peers
                WHERE address = ?
            ",
            )?;
            let mut rows = stmt.query([])?;
            let mut out = vec![];
            while let Some(row) = rows.next()? {
                out.push(Self {
                    address: row.get::<_, String>(0)?.parse()?,
                    last_peer_direction: row.get::<_, String>(1)?.parse()?,
                    block_height: row.get(2)?,
                    first_seen: row
                        .get::<_, Option<i64>>(3)?
                        .map(|x| DateTime::from_utc(NaiveDateTime::from_timestamp(x, 0), Utc)),
                    last_seen: row
                        .get::<_, Option<i64>>(4)?
                        .map(|x| DateTime::from_utc(NaiveDateTime::from_timestamp(x, 0), Utc)),
                    last_connected: row
                        .get::<_, Option<i64>>(5)?
                        .map(|x| DateTime::from_utc(NaiveDateTime::from_timestamp(x, 0), Utc)),
                    blocks_synced_to: row.get(6)?,
                    blocks_synced_from: row.get(7)?,
                    blocks_received_from: row.get(8)?,
                    blocks_sent_to: row.get(9)?,
                    connection_fail_count: row.get(10)?,
                    connection_success_count: row.get(11)?,
                });
            }
            Ok(out)
        })
        .await
    }
}
