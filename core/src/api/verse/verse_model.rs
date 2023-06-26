#[derive(Debug, Default)]
pub struct VerseFilter {
    pub collection: Option<String>,
    pub book: Option<String>,
    pub chapter_number: Option<i32>,
    pub verse_number: Option<i32>,
}
