use std::io;

use async_trait::async_trait;
use mongodb::bson::Document;
use nameof::name_of;
use springtime_di::{component_alias, injectable, Component};

use crate::{error::ToIoError, grammar::Verse, persistence::get_db};

#[derive(Debug, Default)]
pub struct VerseFilter {
    pub collection: Option<String>,
    pub book: Option<String>,
    pub chapter_number: Option<i32>,
    pub verse_number: Option<i32>,
}
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

#[injectable]
#[async_trait]
pub trait VerseService {
    async fn find_one(&self, filters: VerseFilter) -> io::Result<Option<Verse>>;
}

#[derive(Component)]
struct Service;

impl Service {
    async fn get_collection(&self) -> mongodb::Collection<Verse> {
        get_db().await.collection::<Verse>("verses")
    }
}

#[component_alias]
#[async_trait]
impl VerseService for Service {
    async fn find_one(&self, filters: VerseFilter) -> io::Result<Option<Verse>> {
        return self
            .get_collection()
            .await
            .find_one(Into::<Document>::into(filters), None)
            .await
            .map_err(|e| e.to_io());
    }
}
