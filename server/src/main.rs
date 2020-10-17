pub use crate::app::App;
pub use crate::config::Config;
pub use crate::error::{Error, Result};

mod app;
mod config;
mod error;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let app = App::init(Config::read()?).await?;
    app.run().await?;

    Ok(())
}
