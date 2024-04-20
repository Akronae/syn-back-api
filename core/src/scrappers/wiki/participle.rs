use crate::{
    api::lexicon::lexicon_model::{
        LexiconEntryDefinition, VerbInflectionTenses, WordAdjective, WordInflection,
    },
    error::SafeError,
    grammar::{
        Adjective, Contraction, Mood, PartOfSpeech, Tense, Theme, Voice,
    },
    scrappers::wiki::table::parse_declension_table,
    utils::scrapper::select::select,
};

use anyhow::Context;
use scraper::Html;

use super::{
    definition,
    noun::{self},
    page,
    table::{get_words_dialects, ParsedWord, ParsingComp},
    verb,
};

pub struct ScrappedParticiple {
    pub inflections: Vec<WordInflection>,
    pub verb_lemma: String,
}
pub async fn scrap_participle(lemma: &str) -> Result<ScrappedParticiple, SafeError> {
    let doc = page::scrap(lemma).await?;

    let definitions = scrap_participle_defs(&doc)?;
    let verb_lemma;
    let tense;
    let voices;
    if let Some(LexiconEntryDefinition::FormOf(formof)) = definitions.first() {
        verb_lemma = formof.lemma.clone();
        tense = match &formof.text {
            x if x.contains("present") => Tense::Present,
            _ => panic!("cannot find tense in {:?}", formof.text),
        };
        voices = match &formof.text {
            x if x.contains("mediopassive") => vec![Voice::Middle, Voice::Passive],
            _ => panic!("cannot find voices in {:?}", formof.text),
        }
    } else {
        return Err(format!("cannot find verb lemma from {:?}", definitions).into());
    }

    let selector = select(".NavFrame")?;
    let decl_tables = doc.select(&selector);

    let mut inflections = vec![];
    for table in decl_tables {
        let words = parse_declension_table(&table)?
            .iter()
            .map(|x| {
                let mut x = x.clone();
                x.parsing.push(ParsingComp::Tense(tense));
                for v in voices.clone() {
                    x.parsing.push(ParsingComp::Voice(v));
                }
                x
            })
            .collect::<Vec<_>>();
        let mut infl = parsed_words_to_inflection(&words);
        let dialects = get_words_dialects(&words);
        infl.dialects = dialects;
        inflections.push(infl);
    }

    Ok(ScrappedParticiple {
        inflections,
        verb_lemma,
    })
}

pub fn scrap_participle_defs(doc: &Html) -> Result<Vec<LexiconEntryDefinition>, SafeError> {
    let container = doc
        .select(&select("#Participle")?)
        .next()
        .with_context(|| "cannot find participle header".to_string())?;

    let definitions = definition::extract_word_defs(&container)?;

    Ok(definitions)
}

fn parsed_words_to_inflection(words: &[ParsedWord]) -> WordInflection {
    let mut infl = WordInflection::default();

    for word in words {
        fill_pos(word, &mut infl);
    }

    infl
}

fn fill_pos(word: &ParsedWord, pos: &mut WordInflection) {
    if word
        .parsing
        .contains(&ParsingComp::PartOfSpeech(PartOfSpeech::Adverb))
    {
        if pos.adverb.is_none() {
            pos.adverb = Some(Default::default());
        }
        let adverb = pos.adverb.as_mut().unwrap();
        noun::fill_forms(word, adverb);
    } else if let Some(adj) = word.parsing.iter().find_map(|x| match x {
        ParsingComp::PartOfSpeech(PartOfSpeech::Adjective(adj)) => Some(adj),
        _ => None,
    }) {
        if pos.adjective.is_none() {
            pos.adjective = Some(Default::default());
        }
        let adjective = pos.adjective.as_mut().unwrap();
        fill_adjective(word, adjective, adj);
    } else {
        let mut word = word.clone();
        word.parsing.push(ParsingComp::Theme(Theme::Thematic));
        word.parsing
            .push(ParsingComp::Contraction(Contraction::Contracted));
        word.parsing.push(ParsingComp::Mood(Mood::Participle));

        if pos.verb.is_none() {
            pos.verb = Some(Default::default());
        }
        let verb = pos.verb.as_mut().unwrap();
        fill_tenses(&word, verb);
    }
}

fn fill_tenses(word: &ParsedWord, tenses: &mut VerbInflectionTenses) {
    if word.parsing.contains(&ParsingComp::Tense(Tense::Aorist)) {
        if tenses.aorist.is_none() {
            tenses.aorist = Some(Default::default());
        }
        let aorist = tenses.aorist.as_mut().unwrap();
        verb::fill_themes(word, aorist);
    }

    if word.parsing.contains(&ParsingComp::Tense(Tense::Aorist2nd)) {
        if tenses.aorist_2nd.is_none() {
            tenses.aorist_2nd = Some(Default::default());
        }
        let aorist_2nd = tenses.aorist_2nd.as_mut().unwrap();
        verb::fill_themes(word, aorist_2nd);
    }
    if word.parsing.contains(&ParsingComp::Tense(Tense::Future)) {
        if tenses.future.is_none() {
            tenses.future = Some(Default::default());
        }
        let future = tenses.future.as_mut().unwrap();
        verb::fill_themes(word, future);
    }
    if word
        .parsing
        .contains(&ParsingComp::Tense(Tense::FuturePerfect))
    {
        if tenses.future_perfect.is_none() {
            tenses.future_perfect = Some(Default::default());
        }
        let future_perfect = tenses.future_perfect.as_mut().unwrap();
        verb::fill_themes(word, future_perfect);
    }
    if word.parsing.contains(&ParsingComp::Tense(Tense::Imperfect)) {
        if tenses.imperfect.is_none() {
            tenses.imperfect = Some(Default::default());
        }
        let imperfect = tenses.imperfect.as_mut().unwrap();
        verb::fill_themes(word, imperfect);
    }
    if word.parsing.contains(&ParsingComp::Tense(Tense::Perfect)) {
        if tenses.perfect.is_none() {
            tenses.perfect = Some(Default::default());
        }
        let perfect = tenses.perfect.as_mut().unwrap();
        verb::fill_themes(word, perfect);
    }
    if word
        .parsing
        .contains(&ParsingComp::Tense(Tense::Perfect2nd))
    {
        if tenses.perfect_2nd.is_none() {
            tenses.perfect_2nd = Some(Default::default());
        }
        let perfect_2nd = tenses.perfect_2nd.as_mut().unwrap();
        verb::fill_themes(word, perfect_2nd);
    }

    if word
        .parsing
        .contains(&ParsingComp::Tense(Tense::Pluperfect))
    {
        if tenses.pluperfect.is_none() {
            tenses.pluperfect = Some(Default::default());
        }
        let pluperfect = tenses.pluperfect.as_mut().unwrap();
        verb::fill_themes(word, pluperfect);
    }

    if word.parsing.contains(&ParsingComp::Tense(Tense::Present)) {
        if tenses.present.is_none() {
            tenses.present = Some(Default::default());
        }
        let present = tenses.present.as_mut().unwrap();
        verb::fill_themes(word, present);
    }
}

fn fill_adjective(word: &ParsedWord, adjective: &mut WordAdjective, adj: &Adjective) {
    match adj {
        Adjective::Positive => {
            if adjective.positive.is_none() {
                adjective.positive = Some(Default::default());
            }
            let positive = adjective.positive.as_mut().unwrap();
            noun::fill_forms(word, positive);
        }
        Adjective::Comparative => {
            if adjective.comparative.is_none() {
                adjective.comparative = Some(Default::default());
            }
            let comparative = adjective.comparative.as_mut().unwrap();
            noun::fill_forms(word, comparative);
        }
        Adjective::Superlative => {
            if adjective.superlative.is_none() {
                adjective.superlative = Some(Default::default());
            }
            let superlative = adjective.superlative.as_mut().unwrap();
            noun::fill_forms(word, superlative);
        }
    }
}
