use error::SafeError;

mod api;
mod config;
mod error;
mod grammar;
mod log;
mod persistence;
mod scrappers;
mod texts;

#[tokio::main]
async fn main() -> Result<(), SafeError> {
    log::init()?;

    api::init().await?;

    Ok(())
}
