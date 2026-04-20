pub struct ParsedSas7bdat {
    pub metadata: SasMetadata,
    pub reader: BoxedParserDataSource,
    pub header_size: usize,
    pub row_sources: Vec<PageRowSource>,
    pub text_encoding_code: u8,
    pub next_row_source: usize,
    pub pending_rows: VecDeque<ParsedRow>,
    pub next_row_index: usize,
}

impl std::fmt::Debug for ParsedSas7bdat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParsedSas7bdat")
            .field("metadata", &self.metadata)
            .field("header_size", &self.header_size)
            .field("row_sources", &self.row_sources)
            .field("next_row_source", &self.next_row_source)
            .field("pending_rows", &self.pending_rows)
            .field("next_row_index", &self.next_row_index)
            .finish()
    }
}

impl ParsedSas7bdat {
    pub fn new_streaming(
        metadata: SasMetadata,
        reader: BoxedParserDataSource,
        header_size: usize,
        row_sources: Vec<PageRowSource>,
        text_encoding_code: u8,
    ) -> Self {
        Self {
            metadata,
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
