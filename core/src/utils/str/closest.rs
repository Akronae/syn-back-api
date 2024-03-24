use std::{borrow::Borrow, cmp::Ordering};

use strsim::normalized_damerau_levenshtein;

use crate::{borrow::Cow, utils::str::remove_diacritics::remove_diacritics};

pub fn closest(s: Cow<str>, list: &[Cow<str>]) -> Option<Cow<str>> {
    let scores = list
        .iter()
        .map(|x| {
            let dst = normalized_damerau_levenshtein(x, s.borrow());
            let no_dia_dst = normalized_damerau_levenshtein(
                &remove_diacritics(x),
                &remove_diacritics(s.borrow()),
            );
            (x, dst + no_dia_dst)
        })
        .collect::<Vec<_>>();

    return scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
        .map(|x| x.0.clone());
}
