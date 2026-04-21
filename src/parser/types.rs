use super::traits::ParserDataSource;

/// Heap-allocated parser input stream.
pub type BoxedParserDataSource = Box<dyn ParserDataSource>;
