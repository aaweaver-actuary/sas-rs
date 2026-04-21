/// Optional SAS display metadata captured for a column.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColumnMetadata {
    /// Column label.
    pub label: Option<String>,
    /// SAS format name.
    pub format_name: Option<String>,
    /// SAS informat name.
    pub informat_name: Option<String>,
}
