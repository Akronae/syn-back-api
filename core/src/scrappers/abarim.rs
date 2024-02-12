use tracing::log::info;

use crate::{error::SafeError, persistence, texts::Book};

pub mod declension;
pub mod parser;

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let col = "verses";
    let parsed = parser::parse_chapter(1, Book::Matthew).await?;

    persistence::get_db()
        .await?
        .collection(col)
        .insert_many(parsed.verses.to_owned(), None)
        .await?;

    info!("{} verses imported into {col}", parsed.verses.len());

    Ok(())
}
