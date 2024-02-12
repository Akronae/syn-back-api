use crate::{
    api::verse::verse_model::VerseFilter,
    error::{MapErrActix, MapIntoErr, SafeError},
    grammar::Verse,
    texts::{Book, Collection},
};

use super::verse_service::VerseService;

use actix_web::{
    get,
    web::{self, Data, Json, Path},
    Responder,
};
use anyhow::Context;
use paperclip::actix::{api_v2_operation, Apiv2Schema};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Deserialize, Debug)]
struct GetVerseParams {
    collection: Collection,
    book: Book,
    chapter_number: i32,
    verse_number: i32,
}

#[get("/manifest")]
async fn get_manifest() -> actix_web::Result<impl Responder> {
    let manifest = VerseService::get_manifest().await.map_err_actix()?;
    Ok(web::Json(manifest))
}

#[get("/{collection}/{book}/{chapter_number}/{verse_number}")]
async fn get_verse(
    params: Path<GetVerseParams>,
    verse_service: Data<VerseService>,
) -> actix_web::Result<impl Responder> {
    let verse = verse_service
        .find_one(VerseFilter {
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
            .app_data(web::Data::new(VerseService {})),
    );
}
