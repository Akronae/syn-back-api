use crate::{
    api::lexicon::lexicon_model::LexiconFilter, error::MapErrActix,
    utils::extractors::query_nested::QueryNested,
};

use super::lexicon_service::LexiconService;

use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use anyhow::Context;

#[get("/find")]
async fn get_lexicon(
    params: QueryNested<LexiconFilter>,
    lexicon_service: Data<LexiconService>,
) -> actix_web::Result<impl Responder> {
    dbg!(params.clone());
    let lexicon = lexicon_service
        .find_one(LexiconFilter {
            lemma: params.lemma.to_owned(),
            inflection: params.inflection.to_owned(),
        })
        .await
        .map_err_actix()?
        .context("no lexicon entry found")
        .map_err_actix()?;

    Ok(web::Json(lexicon))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("lexicon")
            .service(get_lexicon)
            .app_data(web::Data::new(LexiconService::new())),
    );
}
