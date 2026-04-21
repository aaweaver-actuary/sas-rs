use super::{RowBatchColumn, SasMetadata, SupportedSubset};

/// Shared schema for decoded row batches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowBatchSchema {
    /// Supported subset that produced the batch.
    pub subset: SupportedSubset,
    /// SAS table name.
    pub table_name: String,
    /// SAS file label.
    pub file_label: String,
    /// Output columns derived from the source schema.
    pub columns: Vec<RowBatchColumn>,
}

impl RowBatchSchema {
    /// Build a row-batch schema from parsed dataset metadata.
    pub fn from_metadata(metadata: &SasMetadata) -> Self {
        Self {
            subset: metadata.subset,
            table_name: metadata.table_name.clone(),
            file_label: metadata.file_label.clone(),
            columns: metadata
                .columns
                .iter()
                .enumerate()
                .map(|(source_index, column)| RowBatchColumn::from_source(source_index, column))
                .collect(),
        }
    }

    /// Look up a schema column by name.
    pub fn column(&self, name: &str) -> Option<&RowBatchColumn> {
        self.columns.iter().find(|column| column.name == name)
    }
}
