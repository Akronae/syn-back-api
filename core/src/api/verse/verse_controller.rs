use crate::{
    api::verse::verse_model::VerseFilter,
    error::MapErrActix,
    texts::{Book, Collection},
};

use super::verse_service::VerseService;

use actix_web::{
    get,
    web::{self, Path},
    Responder,
};
use anyhow::Context;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GetVerseParams {
    collection: Collection,
    book: Book,
    chapter_number: u8,
    verse_number: u8,
}

#[get("/manifest")]
async fn get_manifest() -> actix_web::Result<impl Responder> {
    let manifest = VerseService::get_manifest().await.map_err_actix()?;
    Ok(web::Json(manifest))
}

#[get("/{collection}/{book}/{chapter_number}/{verse_number}")]
async fn get_verse(params: Path<GetVerseParams>) -> actix_web::Result<impl Responder> {
    let verse = VerseService::find_one(&VerseFilter {
        collection: Some(params.collection.to_string()),
        book: Some(params.book.to_string()),
        chapter_number: Some(params.chapter_number),
        verse_number: Some(params.verse_number),
    })
    .await
    .map_err_actix()?
    .context("no verse found")
    .map_err_actix()?;

    Ok(web::Json(verse))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("verses")
            .service(get_verse)
            .service(get_manifest)
            .app_data(web::Data::new(VerseService::new())),
    );
}
