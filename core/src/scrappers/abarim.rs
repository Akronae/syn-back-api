use std::error::Error;

use crate::{persistence, texts::Book};

pub mod declension;
pub mod parser;

#[allow(dead_code)]
async fn import(collection: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parsed = parser::parse_chapter(1, Book::Matthew).await?;

    persistence::get_db()
        .await?
        .collection(collection)
        .insert_many(parsed.verses, None)
        .await?;

    Ok(())
}
