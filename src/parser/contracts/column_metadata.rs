#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColumnMetadata {
    pub label: Option<String>,
    pub format_name: Option<String>,
    pub informat_name: Option<String>,
}

