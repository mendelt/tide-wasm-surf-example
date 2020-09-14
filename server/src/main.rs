pub use crate::appstate::AppState;
pub use crate::config::Config;
pub use crate::error::{Error, Result};

mod appstate;
mod config;
mod error;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let mut app = tide::with_state(AppState::init(Config::read()?).await?);

    app.at("/").get(|_| async { Ok("visit /src/*") });
    app.at("/src").serve_dir("static")?;
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
