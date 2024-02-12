use error::SafeError;

mod api;
mod config;
mod error;
mod grammar;
mod log;
mod persistence;
mod scrappers;
mod texts;
mod utils;

#[tokio::main]
async fn main() -> Result<(), SafeError> {
    log::init()?;

    // scrappers::abarim::import().await?;

    api::init().await?;

    Ok(())
}
