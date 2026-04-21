use std::sync::Arc;

use super::parsed_row::ParsedRow;
use super::row_batch_schema::RowBatchSchema;

/// Batch of decoded rows plus the schema they were decoded under.
#[derive(Debug, Clone, PartialEq)]
pub struct RowBatch {
    /// Shared schema for all rows in the batch.
    pub schema: Arc<RowBatchSchema>,
    /// Absolute index of the first row in this batch.
    pub row_index_start: usize,
    /// Decoded rows.
    pub rows: Vec<ParsedRow>,
}
