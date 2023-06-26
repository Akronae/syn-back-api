use std::{env::VarError, str::FromStr};

use anyhow::Context;
use once_cell::sync::OnceCell;

use strum::Display;

use crate::error::SafeError;

static ENV_LOADED: OnceCell<bool> = OnceCell::new();

#[derive(Display, Clone, Copy)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvVar {
    Port,
    RustLog,
    MongoUri,
}

pub struct Config;

impl Config {
    pub fn get<T: FromStr>(self, var: EnvVar) -> Result<T, SafeError> {
        if !(*ENV_LOADED.get_or_init(|| false)) {
            dotenv::dotenv()
                .with_context(|| "Failed to load .env file")
                .unwrap();
        }

        let res = std::env::var(var.to_string());

        match res {
            Ok(value) => match value.parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("Failed to parse env var {var}").into()),
            },
            Err(VarError::NotPresent) | Err(VarError::NotUnicode(_)) => {
                Err(format!("Failed to get env var {var}").into())
            }
        }
    }
}
