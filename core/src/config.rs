use std::{env::VarError, error::Error};

use anyhow::Context;
use once_cell::sync::OnceCell;
use strum::Display;

static ENV_LOADED: OnceCell<bool> = OnceCell::new();

#[derive(Display, Clone, Copy)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvVar {
    Port,
    RustLog,
}

pub struct Config;

impl Config {
    pub fn get(self, var: EnvVar) -> Result<String, Box<dyn Error>> {
        if !(*ENV_LOADED.get_or_init(|| false)) {
            dotenv::dotenv()
                .with_context(|| "Failed to load .env file")
                .unwrap();
        }

        let res = std::env::var(var.to_string());

        match res {
            Ok(value) => Ok(value),
            Err(VarError::NotPresent) | Err(VarError::NotUnicode(_)) => {
                Err(format!("Failed to get env var {var}").into())
            }
        }
    }

    pub fn get_i32(self, var: EnvVar) -> Result<i32, Box<dyn Error>> {
        let str = self.get(var);

        if str.is_err() {
            return Err(str.unwrap_err());
        }

        let str = str.unwrap();

        str
            .parse::<i32>()
            .map_err(|_e| format!("Failed to parse env var {var} to i32").into())
    }
}
