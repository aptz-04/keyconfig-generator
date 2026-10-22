#[derive(Copy, Clone, Default)]
pub struct GridEntry<'a> {
    /// The name of the matrix the grid entry belongs to
    pub belongs_to: Option<&'a str>,
    pub in_use: bool,
}

impl<'a> GridEntry<'a> {
    pub const fn new() -> Self {
        GridEntry {
            belongs_to: None,
            in_use: false,
        }
    }
}
