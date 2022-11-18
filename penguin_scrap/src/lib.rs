#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
pub mod github;
pub mod model;

/// Extractor interface to query a content from a specific source
pub enum PrintFormat {
    Markdown,
}

/// Generic Extractor from a source that can be a
/// web API or a File ecc.
pub trait Extractor {
    type Output = String;
    type Error = String;

    async fn search_new(&self) -> Result<Self::Output, Self::Error>;

    /// Convert the result from the API and return a new
    /// value for the format specified.
    async fn printify(&self, out: &Self::Output, format: PrintFormat) -> String;
}

/// Printer trait that implement the logic
/// print a return given by an Extractor!
pub trait Printer {
    type Input = String;

    /// Build a new printer!
    fn new(created: &str, since: &str, team: &str) -> Self;

    /// Take an input the content that can be
    /// the result of a API call and printify
    /// in a formatted string.
    fn printify(&self, content: &Self::Input) -> String;
}
