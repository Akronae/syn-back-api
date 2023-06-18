mod api;
mod config;
mod error;
mod grammar;
mod log;
mod persistence;
mod scrappers;
mod texts;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::init()?;

    let _app = api::init().await?;

    Ok(())
}
