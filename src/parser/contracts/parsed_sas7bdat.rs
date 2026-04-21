use std::collections::VecDeque;
use std::sync::Arc;

use super::row_batch_schema::RowBatchSchema;
use super::{BoxedParserDataSource, PageRowSource, ParsedRow, SasMetadata};

/// Streaming parsed SAS7BDAT dataset.
pub struct ParsedSas7bdat {
    /// Parsed dataset metadata.
    pub metadata: SasMetadata,
    pub(crate) row_batch_schema: Arc<RowBatchSchema>,
    /// Reader positioned for continued row decoding.
    pub reader: BoxedParserDataSource,
    /// Header size in bytes.
    pub header_size: usize,
    pub(crate) row_sources: Vec<PageRowSource>,
    /// Text encoding code used for string decoding.
    pub text_encoding_code: u8,
    /// Index of the next row source to decode.
    pub next_row_source: usize,
    /// Buffered decoded rows not yet returned in a batch.
    pub pending_rows: VecDeque<ParsedRow>,
    /// Absolute index of the next row to emit.
    pub next_row_index: usize,
}

impl std::fmt::Debug for ParsedSas7bdat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParsedSas7bdat")
            .field("metadata", &self.metadata)
            .field("row_batch_schema", &self.row_batch_schema)
            .field("header_size", &self.header_size)
            .field("row_sources", &self.row_sources)
            .field("next_row_source", &self.next_row_source)
            .field("pending_rows", &self.pending_rows)
            .field("next_row_index", &self.next_row_index)
            .finish()
    }
}

impl ParsedSas7bdat {
    pub(crate) fn new_streaming(
        metadata: SasMetadata,
        reader: BoxedParserDataSource,
        header_size: usize,
        row_sources: Vec<PageRowSource>,
        text_encoding_code: u8,
    ) -> Self {
        let row_batch_schema = Arc::new(RowBatchSchema::from_metadata(&metadata));
        Self {
            metadata,
            row_batch_schema,
            reader,
            header_size,
            row_sources,
            text_encoding_code,
            next_row_source: 0,
            pending_rows: VecDeque::new(),
            next_row_index: 0,
        }
    }
}
