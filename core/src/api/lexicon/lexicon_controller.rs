use crate::{
    api::lexicon::lexicon_model::LexiconFilter,
    error::MapErrActix,
};

use super::{lexicon_service::LexiconService};

use actix_web::{
    get,
    web::{self, Data, Path},
    Responder,
};
use anyhow::Context;



#[get("/{lemma}")]
async fn get_lexicon(
    params: Path<LexiconFilter>,
    lexicon_service: Data<LexiconService>,
) -> actix_web::Result<impl Responder> {
    let lexicon = lexicon_service
        .find_one(LexiconFilter {
            lemma: params.lemma.to_owned(),
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
