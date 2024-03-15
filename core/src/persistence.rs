use anyhow::Context;
use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;

use crate::{
    config::{Config, EnvVar},
    error::SafeError,
};

static DB: OnceCell<Database> = OnceCell::new();
pub async fn get_db() -> Result<Database, SafeError> {
    if DB.get().is_none() {
        let mut client_options = ClientOptions::parse(Config.get::<String>(EnvVar::MongoUri)?)
            // .await
            .with_context(|| "Failed to parse MongoDB URI")?;

        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options).expect("Failed to initialize client.");
        let db = client.database("syn-text-api");
        DB.set(db)
            .map_err(|_| "Failed to set DB instance in OnceCell")?;
    }

    Ok(DB
        .get()
        .with_context(|| "could not get DB instance from OnceCell")?
        .clone())
}
