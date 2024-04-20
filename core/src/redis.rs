use anyhow::Context;
use once_cell::sync::OnceCell;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};

use crate::{config::EnvVar, error::SafeError};

static CONN: OnceCell<MultiplexedConnection> = OnceCell::new();

async fn get_redis() -> Result<MultiplexedConnection, SafeError> {
    if CONN.get().is_none() {
        let client = Client::open(EnvVar::RedisUri.get::<String>()?).unwrap();
        let conn = client.get_multiplexed_tokio_connection().await?;
        CONN.set(conn)
            .map_err(|_| "Failed to set redis instance in OnceCell")?;
    }
    let conn = CONN
        .get()
        .with_context(|| "could not get redis instance from OnceCell")?;
    Ok(conn.clone())
}

fn get_redis_key(key: &str) -> String {
    format!(
        "{}:{}",
        EnvVar::RedisKeyPrefix.get::<String>().unwrap(),
        key
    )
}

pub async fn set(key: &str, value: &str) -> Result<(), SafeError> {
    let mut conn = get_redis().await?;
    conn.set(get_redis_key(key), value).await?;
    Ok(())
}

pub async fn get(key: &str) -> Result<Option<String>, SafeError> {
    let mut conn = get_redis().await?;
    let value = conn.get(get_redis_key(key)).await?;

    Ok(match value {
        redis::Value::Data(data) => Some(String::from_utf8(data)?),
        _ => None,
    })
}
