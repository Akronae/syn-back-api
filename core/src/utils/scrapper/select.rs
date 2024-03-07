use scraper::Selector;

use crate::error::{MapIntoErr, SafeError};

pub fn select(str: &str) -> Result<Selector, SafeError> {
    Selector::parse(str).map_into_err()
}
