use crate::{Config, Result};
use sqlx::PgPool;
use tide::Request;

#[derive(Debug, Clone)]
pub struct AppState {
    database: PgPool,
    config: Config,
}

impl AppState {
    pub async fn init(config: Config) -> Result<Self> {
        Ok(AppState {
            database: PgPool::connect(&config.database_url).await?,
            config,
        })
    }
}

trait AppStateExt {
    fn database(&self) -> PgPool;
}

impl AppStateExt for Request<AppState> {
    fn database(&self) -> PgPool {
        todo!();
    }
}
