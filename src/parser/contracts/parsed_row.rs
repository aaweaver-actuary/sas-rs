use super::ParsedValue;

/// Decoded logical row returned by the parser.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedRow {
    /// Decoded row values in source column order.
    pub values: Vec<ParsedValue>,
}
