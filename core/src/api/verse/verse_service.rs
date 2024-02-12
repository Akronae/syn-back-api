use std::io;

use mongodb::{bson::Document, Collection};
use nameof::name_of;

use crate::{
    error::{MapErrSafe, MapIntoErr, SafeError},
    grammar::Verse,
    persistence::get_db,
};

use super::verse_model::VerseFilter;

impl From<VerseFilter> for Document {
    fn from(value: VerseFilter) -> Self {
        let mut doc = Document::new();

        if let Some(collection) = value.collection {
            doc.insert(name_of!(collection), collection);
        }
        if let Some(book) = value.book {
            doc.insert(name_of!(book), book);
        }
        if let Some(chapter_number) = value.chapter_number {
            doc.insert(name_of!(chapter_number), chapter_number);
        }
        if let Some(verse_number) = value.verse_number {
            doc.insert(name_of!(verse_number), verse_number);
        }

        doc
    }
}

pub struct VerseService;

impl VerseService {
    async fn get_collection(&self) -> Result<Collection<Verse>, SafeError> {
        Ok(get_db().await?.collection::<Verse>("verses"))
    }

    pub async fn find_one(&self, filters: VerseFilter) -> Result<Option<Verse>, SafeError> {
        self.get_collection()
            .await?
            .find_one(Into::<Document>::into(filters), None)
            .await
            .map_err_safe()
    }
}
