use std::io::{Read, Seek};

use super::{ParsedSas7bdat, ParserError, ParserInput};

/// Parser abstraction for SAS7BDAT inputs.
pub trait Sas7bdatParser {
    /// Parse a source into a streaming decoded dataset.
    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError>;
}

/// Read/seek data source accepted by the parser.
pub trait ParserDataSource: Read + Seek + Send {}

impl<T> ParserDataSource for T where T: Read + Seek + Send {}
