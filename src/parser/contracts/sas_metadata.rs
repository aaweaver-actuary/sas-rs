use super::{SasColumn, SupportedSubset};

/// Parsed dataset-level metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasMetadata {
    /// Supported subset used to decode the file.
    pub subset: SupportedSubset,
    /// SAS table name.
    pub table_name: String,
    /// SAS file label.
    pub file_label: String,
    /// Declared row count.
    pub row_count: usize,
    /// Declared row length.
    pub row_length: usize,
    /// Declared page size.
    pub page_size: usize,
    /// Declared page count.
    pub page_count: usize,
    /// Parsed column descriptors.
    pub columns: Vec<SasColumn>,
}
