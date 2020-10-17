use crate::{Config, Result};
use sqlx::SqlitePool;
use tide::Request;

#[derive(Debug, Clone)]
pub struct App {
    database: SqlitePool,
    config: Config,
}

impl App {
    pub async fn init(config: Config) -> Result<Self> {
        Ok(App {
            database: SqlitePool::connect(&config.database_url).await?,
            config,
        })
    }

    pub async fn run(self) -> Result<()> {
        let mut server = tide::with_state(self);

        server.at("/").get(|_| async { Ok("visit /src/*") });
        server.at("/src").serve_dir("static")?;
        server.listen("127.0.0.1:8080").await?;

        Ok(())
    }
}

trait AppStateExt {
    fn database(&self) -> &SqlitePool;
}

impl AppStateExt for Request<App> {
    fn database(&self) -> &SqlitePool {
        &self.state().database
    }
}
