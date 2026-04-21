use std::io::Cursor;

use super::BoxedParserDataSource;
use crate::parser::traits::ParserDataSource;

/// Named parser input source and its readable stream.
pub struct ParserInput<'a> {
    /// Source name used in diagnostics.
    pub source_name: &'a str,
    /// Stream from which bytes are read.
    pub reader: BoxedParserDataSource,
}

impl<'a> ParserInput<'a> {
    /// Build a parser input from an already boxed data source.
    pub fn new(source_name: &'a str, reader: BoxedParserDataSource) -> Self {
        Self {
            source_name,
            reader,
        }
    }

    /// Build a parser input backed by an in-memory byte buffer.
    pub fn from_bytes(source_name: &'a str, bytes: Vec<u8>) -> Self {
        Self::from_reader(source_name, Cursor::new(bytes))
    }

    /// Build a parser input from any read/seek source.
    pub fn from_reader<R>(source_name: &'a str, reader: R) -> Self
    where
        R: ParserDataSource + 'static,
    {
        Self::new(source_name, Box::new(reader))
    }
}
