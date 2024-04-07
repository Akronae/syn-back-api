pub mod details;
mod noun;
mod page;
pub mod parser;

use crate::error::SafeError;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    Ok(())
}
