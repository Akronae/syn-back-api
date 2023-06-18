

use axum::extract::Path;
use mongodb::bson::doc;
use serde::Deserialize;
use springtime_di::{instance_provider::ComponentInstancePtr, Component};
use springtime_web_axum::controller;



use super::verse_service::{VerseFilter, VerseService};

#[derive(Component)]
struct VersesController {
    verse_service: ComponentInstancePtr<dyn VerseService + Send + Sync>,
}

#[derive(Deserialize)]
struct GetVerseParams {
    collection: String,
    book: String,
    chapter_number: i32,
    verse_number: i32,
}

#[controller(path = "/verse")]
impl VersesController {
    #[get("/")]
    async fn hello_world(&self) -> &'static str {
        "Hello world!"
    }

    #[get("/:collection/:book/:chapter_number/:verse_number")]
    async fn get_verse(&self, Path(params): Path<GetVerseParams>) -> String {
        let a = self
            .verse_service
            .find_one(VerseFilter {
                collection: Some(params.collection),
                book: Some(params.book),
                chapter_number: Some(params.chapter_number),
                verse_number: Some(params.verse_number),
                ..VerseFilter::default()
            })
            .await;

        format!("{:?}", a)
    }
}
