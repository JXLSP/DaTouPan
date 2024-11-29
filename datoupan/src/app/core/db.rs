use std::{sync::OnceLock, time::Duration};

use config::Config;
use sea_orm::{ ConnectOptions, DatabaseConnection, Database };

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(cfg: &Config) {
    let conn = new(cfg, "db")
        .await
        .unwrap_or_else(|e| panic!("Failed to create database connection: {}", e));

    let _ = DB.set(conn);
}

pub async fn new(cfg: &Config, key: &str) -> anyhow::Result<DatabaseConnection> {
    let opts = ConnectOptions::new(cfg.get_string(&format!("{}.dsn", key)))?;

    let min_connections = cfg.get_int(&format!("{}.min_connections", key)).unwrap_or(10);
    let max_connections = cfg.get_int(&format!("{}.max_connections", key)).unwrap_or(10);
    let timeout = cfg.get_int(&format!("{}.timeout", key)).unwrap_or(5000);

    opts.min_connections(min_connections as u32)
        .max_connections(max_connections as u32)
        .idle_timeout(Duration::from_secs(timeout as u64));

    let conn = Database::connect(opts).await?;
    let _ = conn.ping().await?;
    Ok(conn)
}
