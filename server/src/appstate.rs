use crate::{Config, Result};
use sqlx::SqlitePool;
use tide::Request;

#[derive(Debug, Clone)]
pub struct AppState {
    database: SqlitePool,
    config: Config,
}

impl AppState {
    pub async fn init(config: Config) -> Result<Self> {
        Ok(AppState {
            database: SqlitePool::connect(&config.database_url).await?,
            config,
        })
    }
}

trait AppStateExt {
    fn database(&self) -> SqlitePool;
}

impl AppStateExt for Request<AppState> {
    fn database(&self) -> SqlitePool {
        todo!();
    }
}
