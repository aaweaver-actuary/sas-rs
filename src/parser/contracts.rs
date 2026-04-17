pub use super::UnsupportedFeature;

use std::collections::VecDeque;
use std::io::{Cursor, Read, Seek};

pub trait ParserDataSource: Read + Seek + Send {}

impl<T> ParserDataSource for T where T: Read + Seek + Send {}

pub type BoxedParserDataSource = Box<dyn ParserDataSource>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordSize {
    Bit32,
    Bit64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    None,
    Row,
    Binary,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedSubset {
    pub name: &'static str,
    pub word_size: WordSize,
    pub endianness: Endianness,
    pub compression: CompressionMode,
}

pub const SUPPORTED_SUBSET: SupportedSubset = SupportedSubset {
    name: "sas7bdat-64le-uncompressed-v1",
    word_size: WordSize::Bit64,
    endianness: Endianness::Little,
    compression: CompressionMode::None,
};

pub struct ParserInput<'a> {
    pub source_name: &'a str,
    pub reader: BoxedParserDataSource,
}

impl<'a> ParserInput<'a> {
    pub fn new(source_name: &'a str, reader: BoxedParserDataSource) -> Self {
        Self {
            source_name,
            reader,
        }
    }

    pub fn from_bytes(source_name: &'a str, bytes: Vec<u8>) -> Self {
        Self::from_reader(source_name, Cursor::new(bytes))
    }

    pub fn from_reader<R>(source_name: &'a str, reader: R) -> Self
    where
        R: ParserDataSource + 'static,
    {
        Self::new(source_name, Box::new(reader))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnKind {
    Numeric,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticTypeHint {
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColumnMetadata {
    pub label: Option<String>,
    pub format_name: Option<String>,
    pub informat_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasColumn {
    pub name: String,
    pub kind: ColumnKind,
    pub offset: usize,
    pub width: usize,
    pub semantic_type: SemanticTypeHint,
    pub metadata: ColumnMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericValue {
    Float64(f64),
    DeferredBytes {
        width_bytes: usize,
        raw_bytes: Vec<u8>,
    },
}

impl NumericValue {
    pub fn deferred_bytes(raw_bytes: Vec<u8>) -> Self {
        Self::DeferredBytes {
            width_bytes: raw_bytes.len(),
            raw_bytes,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float64(value) => Some(*value),
            Self::DeferredBytes { .. } => None,
        }
    }

    pub fn width_bytes(&self) -> usize {
        match self {
            Self::Float64(_) => 8,
            Self::DeferredBytes { width_bytes, .. } => *width_bytes,
        }
    }

    pub fn raw_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Float64(_) => None,
            Self::DeferredBytes { raw_bytes, .. } => Some(raw_bytes.as_slice()),
        }
    }
}

impl From<f64> for NumericValue {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    Numeric(NumericValue),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedRow {
    pub values: Vec<ParsedValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowBatch {
    pub row_index_start: usize,
    pub rows: Vec<ParsedRow>,
}

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

pub struct ParsedSas7bdat {
    pub metadata: SasMetadata,
    pub(crate) reader: BoxedParserDataSource,
    pub(crate) header_size: usize,
    pub(crate) data_pages: Vec<usize>,
    pub(crate) next_data_page: usize,
    pub(crate) pending_rows: VecDeque<ParsedRow>,
    pub(crate) next_row_index: usize,
}

impl std::fmt::Debug for ParsedSas7bdat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParsedSas7bdat")
            .field("metadata", &self.metadata)
            .field("header_size", &self.header_size)
            .field("data_pages", &self.data_pages)
            .field("next_data_page", &self.next_data_page)
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
        data_pages: Vec<usize>,
    ) -> Self {
        Self {
            metadata,
            reader,
            header_size,
            data_pages,
            next_data_page: 0,
            pending_rows: VecDeque::new(),
            next_row_index: 0,
        }
    }
}
