use super::contracts::{self, CompressionMode, SasMetadata};
use super::subheader::finalize_columns;
use super::{ParserError, SasLayout, TextRef};

#[derive(Debug, Clone, Default)]
pub(crate) struct ColumnMetadataState {
    pub(crate) name_ref: Option<TextRef>,
    pub(crate) kind: Option<super::contracts::ColumnKind>,
    pub(crate) offset: Option<usize>,
    pub(crate) width: Option<usize>,
    pub(crate) label_ref: Option<TextRef>,
    pub(crate) format_ref: Option<TextRef>,
    pub(crate) format_width: Option<u16>,
    pub(crate) format_digits: Option<u16>,
    pub(crate) informat_name: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SasMetadataAccumulator {
    pub(crate) table_name: String,
    pub(crate) file_label: String,
    pub(crate) row_count: usize,
    pub(crate) row_length: usize,
    pub(crate) page_row_count: usize,
    pub(crate) page_size: usize,
    pub(crate) page_count: usize,
    pub(crate) text_encoding_code: u8,
    pub(crate) declared_column_count: Option<usize>,
    pub(crate) parsed_name_count: usize,
    pub(crate) parsed_attr_count: usize,
    pub(crate) parsed_format_count: usize,
    pub(crate) text_blobs: Vec<Vec<u8>>,
    pub(crate) columns: Vec<ColumnMetadataState>,
    pub(crate) compression: CompressionMode,
}

impl SasMetadataAccumulator {
    pub(crate) fn new(
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

    pub(crate) fn compression(&self) -> CompressionMode {
        self.compression
    }

    pub(crate) fn page_row_count(&self) -> usize {
        self.page_row_count
    }

    pub(crate) fn row_length(&self) -> usize {
        self.row_length
    }

    pub(crate) fn into_metadata(self, layout: SasLayout) -> Result<SasMetadata, ParserError> {
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
