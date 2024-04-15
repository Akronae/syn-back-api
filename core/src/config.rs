use std::{env::VarError, str::FromStr};

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
    RedisUri,
    RedisKeyPrefix,
}

impl EnvVar {
    pub fn get<T: FromStr>(&self) -> Result<T, SafeError> {
        if !(*ENV_LOADED.get_or_init(|| false)) {
            dotenv::from_filename(".env.local").ok();
            dotenv::from_filename(".env").ok();
        }

        let res = std::env::var(self.to_string());

        match res {
            Ok(value) => match value.parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("Failed to parse env var {self}").into()),
            },
            Err(VarError::NotPresent) | Err(VarError::NotUnicode(_)) => {
                Err(format!("Failed to get env var {self}").into())
            }
        }
    }
}
