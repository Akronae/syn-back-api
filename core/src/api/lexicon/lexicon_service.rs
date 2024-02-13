use crate::error::SafeError;

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
        LexiconRepo::find_one(filter).await
    }
}
