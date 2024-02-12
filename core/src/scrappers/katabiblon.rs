pub mod details;
pub mod inflections;
pub mod parser;

use tracing::log::info;

use crate::{
    error::SafeError,
    grammar::{Case, Declension, Gender, Noun, Number, PartOfSpeech},
};

#[allow(dead_code)]
pub async fn import() -> Result<(), SafeError> {
    let parsed = parser::parse_word(
        "βιβλος",
        &Declension {
            case: Some(Case::Nominative),
            number: Some(Number::Singular),
            gender: Some(Gender::Feminine),
            ..Declension::partial_default(PartOfSpeech::Noun(Noun::Common))
        },
    )
    .await?;

    dbg!(parsed);

    Ok(())
}
