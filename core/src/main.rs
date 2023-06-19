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

    let _app = api::init().await?;

    Ok(())
}
