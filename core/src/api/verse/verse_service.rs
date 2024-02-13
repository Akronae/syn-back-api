use serde::Serialize;

use crate::{error::SafeError, grammar::Verse};

use super::{verse_model::VerseFilter, verse_repo::VerseRepo};

#[derive(Serialize)]
pub struct ManifestChapter {
    pub number: i32,
    pub verses: i32,
}

#[derive(Serialize)]
pub struct ManifestBook {
    pub name: String,
    pub chapters: Vec<ManifestChapter>,
}

#[derive(Serialize)]
pub struct ManifestCollection {
    pub name: String,
    pub books: Vec<ManifestBook>,
}

#[derive(Serialize)]
pub struct Manifest {
    pub collections: Vec<ManifestCollection>,
}

pub struct VerseService {
    repo: VerseRepo,
}

impl VerseService {
    pub fn new() -> VerseService {
        VerseService { repo: VerseRepo {} }
    }

    pub async fn find_one(filter: &VerseFilter) -> Result<Option<Verse>, SafeError> {
        VerseRepo::find_one(filter).await
    }

    pub async fn get_manifest() -> Result<Manifest, SafeError> {
        Ok(Manifest {
            collections: Vec::<ManifestCollection>::from([ManifestCollection {
                name: "new_testament".to_owned(),
                books: Vec::<ManifestBook>::from([ManifestBook {
                    name: "matthew".to_owned(),
                    chapters: Vec::<ManifestChapter>::from([
                        ManifestChapter {
                            number: 1,
                            verses: 25,
                        },
                        ManifestChapter {
                            number: 2,
                            verses: 23,
                        },
                        ManifestChapter {
                            number: 3,
                            verses: 17,
                        },
                        ManifestChapter {
                            number: 4,
                            verses: 25,
                        },
                        ManifestChapter {
                            number: 5,
                            verses: 48,
                        },
                        ManifestChapter {
                            number: 6,
                            verses: 34,
                        },
                        ManifestChapter {
                            number: 7,
                            verses: 29,
                        },
                        ManifestChapter {
                            number: 8,
                            verses: 34,
                        },
                        ManifestChapter {
                            number: 9,
                            verses: 38,
                        },
                        ManifestChapter {
                            number: 10,
                            verses: 42,
                        },
                        ManifestChapter {
                            number: 11,
                            verses: 30,
                        },
                        ManifestChapter {
                            number: 12,
                            verses: 50,
                        },
                        ManifestChapter {
                            number: 13,
                            verses: 58,
                        },
                        ManifestChapter {
                            number: 14,
                            verses: 36,
                        },
                        ManifestChapter {
                            number: 15,
                            verses: 39,
                        },
                        ManifestChapter {
                            number: 16,
                            verses: 28,
                        },
                        ManifestChapter {
                            number: 17,
                            verses: 27,
                        },
                        ManifestChapter {
                            number: 18,
                            verses: 35,
                        },
                        ManifestChapter {
                            number: 19,
                            verses: 30,
                        },
                        ManifestChapter {
                            number: 20,
                            verses: 34,
                        },
                        ManifestChapter {
                            number: 21,
                            verses: 46,
                        },
                        ManifestChapter {
                            number: 22,
                            verses: 46,
                        },
                        ManifestChapter {
                            number: 23,
                            verses: 39,
                        },
                        ManifestChapter {
                            number: 24,
                            verses: 51,
                        },
                        ManifestChapter {
                            number: 25,
                            verses: 46,
                        },
                        ManifestChapter {
                            number: 26,
                            verses: 75,
                        },
                        ManifestChapter {
                            number: 27,
                            verses: 66,
                        },
                        ManifestChapter {
                            number: 28,
                            verses: 20,
                        },
                    ]),
                }]),
            }]),
        })
    }
}
