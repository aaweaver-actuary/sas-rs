#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasMetadata {
    pub subset: SupportedSubset,
    pub table_name: String,
    pub file_label: String,
    pub row_count: usize,
    pub row_length: usize,
    pub page_size: usize,
    pub page_count: usize,
    pub columns: Vec<SasColumn>,
}
