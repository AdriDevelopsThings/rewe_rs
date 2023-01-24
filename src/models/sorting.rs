pub enum Sorting {
    RelevanceDescending,
    RelevanceAscending
}

impl Sorting {
    pub fn to_str(&self) -> &str {
        match self {
            Sorting::RelevanceDescending => "RELEVANCE_DESC",
            Sorting::RelevanceAscending => "RELEVANCE_ASC"
        }
    }
}

impl Default for Sorting {
    fn default() -> Self {
        Self::RelevanceDescending
    }
}