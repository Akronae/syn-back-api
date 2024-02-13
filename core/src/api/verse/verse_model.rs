use mongodb::bson::Document;
use nameof::name_of;

use crate::{grammar::Verse, utils::str::camel_case::CamelCase};

#[derive(Debug, Default)]
pub struct VerseFilter {
    pub collection: Option<String>,
    pub book: Option<String>,
    pub chapter_number: Option<u8>,
    pub verse_number: Option<u8>,
}

impl From<&Verse> for VerseFilter {
    fn from(verse: &Verse) -> Self {
        Self {
            collection: Some(verse.collection.to_string()),
            book: Some(verse.book.to_string()),
            chapter_number: Some(verse.chapter_number),
            verse_number: Some(verse.verse_number),
        }
    }
}

impl From<&VerseFilter> for Document {
    fn from(value: &VerseFilter) -> Self {
        let mut doc = Document::new();

        if let Some(collection) = &value.collection {
            doc.insert(name_of!(collection).camel_case(), collection);
        }
        if let Some(book) = &value.book {
            doc.insert(name_of!(book).camel_case(), book);
        }
        if let Some(chapter_number) = value.chapter_number {
            doc.insert(name_of!(chapter_number).camel_case(), chapter_number as i32);
        }
        if let Some(verse_number) = value.verse_number {
            doc.insert(name_of!(verse_number).camel_case(), verse_number as i32);
        }

        doc
    }
}
