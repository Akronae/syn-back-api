

use strum::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum Collection {
    NewTestament,
}

#[derive(Debug, Display, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum Book {
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    Corinthians1,
    Corinthians2,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    Thessalonians1,
    Thessalonians2,
    Timothy1,
    Timothy2,
    Titus,
    Philemon,
    Hebrews,
    James,
    Peter1,
    Peter2,
    John1,
    John2,
    John3,
    Jude,
    Revelation,
}
