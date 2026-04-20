#[derive(Debug, Clone)]
pub struct SasMetadataAccumulator {
    table_name: String,
    file_label: String,
    row_count: usize,
    row_length: usize,
    page_row_count: usize,
    page_size: usize,
    page_count: usize,
    text_encoding_code: u8,
    declared_column_count: Option<usize>,
    parsed_name_count: usize,
    parsed_attr_count: usize,
    parsed_format_count: usize,
    text_blobs: Vec<Vec<u8>>,
    columns: Vec<ColumnMetadataState>,
    compression: CompressionMode,
}

impl SasMetadataAccumulator {
    pub fn new(
        table_name: String,
        page_size: usize,
        page_count: usize,
        text_encoding_code: u8,
    ) -> Self {
        Self {
            table_name,
            file_label: String::new(),
            row_count: 0,
            row_length: 0,
            page_row_count: 0,
            page_size,
            page_count,
            text_encoding_code,
            declared_column_count: None,
            parsed_name_count: 0,
            parsed_attr_count: 0,
            parsed_format_count: 0,
            text_blobs: Vec::new(),
            columns: Vec::new(),
            compression: CompressionMode::None,
        }
    }

    pub fn compression(&self) -> CompressionMode {
        self.compression
    }

    pub fn page_row_count(&self) -> usize {
        self.page_row_count
    }

    pub fn row_length(&self) -> usize {
        self.row_length
    }

    pub fn into_metadata(self, layout: SasLayout) -> Result<SasMetadata, ParserError> {
        let columns = finalize_columns(&self)?;
        Ok(SasMetadata {
            subset: contracts::supported_subset(
                layout.word_size,
                layout.endianness,
                self.compression,
            ),
            table_name: self.table_name,
            file_label: self.file_label,
            row_count: self.row_count,
            row_length: self.row_length,
            page_size: self.page_size,
            page_count: self.page_count,
            columns,
        })
    }
}
