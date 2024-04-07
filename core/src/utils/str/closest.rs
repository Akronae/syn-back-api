use std::{borrow::Borrow, cmp::Ordering};

use strsim::normalized_damerau_levenshtein;

use crate::{borrow::Cow, utils::str::remove_diacritics::remove_diacritics};

pub struct Scored {
    pub value: Cow<str>,
    pub score: f64,
}

pub fn similarity_score(s1: Cow<str>, s2: Cow<str>) -> f64 {
    let dst = normalized_damerau_levenshtein(&s1, &s2);
    let no_dia_dst =
        normalized_damerau_levenshtein(&remove_diacritics(&s1), &remove_diacritics(&s2));
    return dst + no_dia_dst;
}

pub fn closest_with_score(s: Cow<str>, list: &[Cow<str>]) -> Vec<Scored> {
    let mut scores = list
        .iter()
        .map(|x| (x, similarity_score(x.clone(), s.clone())))
        .collect::<Vec<_>>();

    scores.sort_by(|b, a| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    return scores
        .iter()
        .map(|x| Scored {
            value: x.0.clone(),
            score: x.1,
        })
        .collect();
}

pub fn closest(s: Cow<str>, list: &[Cow<str>]) -> Vec<Cow<str>> {
    return closest_with_score(s, list)
        .iter()
        .map(|x| x.value.clone())
        .collect();
}
