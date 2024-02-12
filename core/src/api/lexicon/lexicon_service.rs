use mongodb::{bson::Document, Collection};
use nameof::name_of;
use serde::Serialize;

use crate::{
    error::{MapErrSafe, SafeError},
    persistence::get_db,
    utils::str::camel_case::CamelCase,
};

use super::{
    lexicon_model::{LexiconEntry, LexiconFilter},
    lexicon_repo::LexiconRepo,
};

pub struct LexiconService {
    repo: LexiconRepo,
}

impl LexiconService {
    pub fn new() -> Self {
        LexiconService {
            repo: LexiconRepo {},
        }
    }

    pub async fn find_one(&self, filter: LexiconFilter) -> Result<Option<LexiconEntry>, SafeError> {
        return self.repo.find_one(filter).await;
    }
}
