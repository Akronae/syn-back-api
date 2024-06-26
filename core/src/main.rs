use error::SafeError;

mod api;
mod borrow;
mod config;
mod error;
mod grammar;
mod infl;
mod log;
mod persistence;
mod redis;
mod request;
mod scrappers;
mod task;
mod texts;
mod utils;

#[tokio::main]
async fn main() -> Result<(), SafeError> {
    log::init()?;

    // scrappers::abarim::import().await?;
    // scrappers::katabiblon::import().await?;
    scrappers::wiki::import().await?;

    api::init().await?;

    Ok(())
}
