use crate::api::verse::verse_model::VerseFilter;

use super::verse_service::VerseService;

use actix_web::{
    get,
    web::{self, Data, Path},
    Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetVerseParams {
    collection: String,
    book: String,
    chapter_number: i32,
    verse_number: i32,
}

#[get("/{collection}/{book}/{chapter_number}/{verse_number}")]
async fn get_verse(
    params: Path<GetVerseParams>,
    verse_service: Data<VerseService>,
) -> actix_web::Result<impl Responder> {
    let verse = verse_service
        .find_one(VerseFilter {
            collection: Some(params.collection.to_owned()),
            book: Some(params.book.to_owned()),
            chapter_number: Some(params.chapter_number),
            verse_number: Some(params.verse_number),
        })
        .await?;

    Ok(web::Json(verse))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("verses")
            .service(get_verse)
            .app_data(web::Data::new(VerseService {})),
    );
}
