use anyhow::Context;

use scraper::{Element, ElementRef};

use crate::{
    api::lexicon::lexicon_model::{DefinitionFormOf, LexiconEntryDefinition},
    borrow::Cow,
    error::SafeError,
    utils::scrapper::{
        filter_by_tag::FilterByTag,
        select::{select},
    },
};

pub fn extract_word_defs(
    section_header: &ElementRef,
) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let ol = section_header
        .parent_element()
        .with_context(|| "cannot find header parent".to_string())?
        .next_siblings()
        .filter_by_tag("ol")
        .next()
        .with_context(|| "cannot find header next <ol> sibling".to_string())?;
    let lis = ol.children().filter_by_tag("li");

    let mut definitions = Vec::new();
    for li in lis {
        if let Some(formof) = ElementRef::wrap(li)
            .unwrap()
            .select(&select(".form-of-definition")?)
            .next()
        {
            let formof_lemma = formof
                .select(&select(".form-of-definition-link .Polyt a")?)
                .next()
                .with_context(|| "cannot find form-of-definition-link".to_string())?
                .attr("title")
                .with_context(|| "no title attribute")?
                .trim()
                .to_string();
            let text = formof.text().collect::<String>();
            definitions.push(LexiconEntryDefinition::FormOf(DefinitionFormOf {
                lemma: formof_lemma,
                text,
            }));
        } else {
            let text = ElementRef::wrap(li)
                .unwrap()
                .text()
                .collect::<Cow<str>>()
                .trim()
                .to_string();
            definitions.push(LexiconEntryDefinition::Litteral(text));
        }
    }
    Ok(definitions)
}
