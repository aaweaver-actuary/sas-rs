#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    Numeric(NumericValue),
    String(String),
}
