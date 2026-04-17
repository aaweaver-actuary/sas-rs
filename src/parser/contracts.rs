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

pub const SUPPORTED_SUBSET_NAME: &str = "sas7bdat-64le-uncompressed-v1";
const SUPPORTED_SUBSET_NAME_32LE: &str = "sas7bdat-32le-uncompressed-v1";
const SUPPORTED_SUBSET_NAME_32BE: &str = "sas7bdat-32be-uncompressed-v1";
const SUPPORTED_SUBSET_NAME_64BE: &str = "sas7bdat-64be-uncompressed-v1";
const SUPPORTED_SUBSET_NAME_32LE_ROW: &str = "sas7bdat-32le-row-compressed-v1";
const SUPPORTED_SUBSET_NAME_32BE_ROW: &str = "sas7bdat-32be-row-compressed-v1";
const SUPPORTED_SUBSET_NAME_64LE_ROW: &str = "sas7bdat-64le-row-compressed-v1";
const SUPPORTED_SUBSET_NAME_64BE_ROW: &str = "sas7bdat-64be-row-compressed-v1";
const SUPPORTED_SUBSET_NAME_32LE_BINARY: &str = "sas7bdat-32le-binary-compressed-v1";
const SUPPORTED_SUBSET_NAME_32BE_BINARY: &str = "sas7bdat-32be-binary-compressed-v1";
const SUPPORTED_SUBSET_NAME_64LE_BINARY: &str = "sas7bdat-64le-binary-compressed-v1";
const SUPPORTED_SUBSET_NAME_64BE_BINARY: &str = "sas7bdat-64be-binary-compressed-v1";
const SUPPORTED_SUBSET_NAME_32LE_UNKNOWN: &str = "sas7bdat-32le-unknown-compression-v1";
const SUPPORTED_SUBSET_NAME_32BE_UNKNOWN: &str = "sas7bdat-32be-unknown-compression-v1";
const SUPPORTED_SUBSET_NAME_64LE_UNKNOWN: &str = "sas7bdat-64le-unknown-compression-v1";
const SUPPORTED_SUBSET_NAME_64BE_UNKNOWN: &str = "sas7bdat-64be-unknown-compression-v1";

fn supported_subset_name(
    word_size: WordSize,
    endianness: Endianness,
    compression: CompressionMode,
) -> &'static str {
    match (word_size, endianness, compression) {
        (WordSize::Bit32, Endianness::Little, CompressionMode::None) => SUPPORTED_SUBSET_NAME_32LE,
        (WordSize::Bit32, Endianness::Big, CompressionMode::None) => SUPPORTED_SUBSET_NAME_32BE,
        (WordSize::Bit64, Endianness::Little, CompressionMode::None) => SUPPORTED_SUBSET_NAME,
        (WordSize::Bit64, Endianness::Big, CompressionMode::None) => SUPPORTED_SUBSET_NAME_64BE,
        (WordSize::Bit32, Endianness::Little, CompressionMode::Row) => {
            SUPPORTED_SUBSET_NAME_32LE_ROW
        }
        (WordSize::Bit32, Endianness::Big, CompressionMode::Row) => SUPPORTED_SUBSET_NAME_32BE_ROW,
        (WordSize::Bit64, Endianness::Little, CompressionMode::Row) => {
            SUPPORTED_SUBSET_NAME_64LE_ROW
        }
        (WordSize::Bit64, Endianness::Big, CompressionMode::Row) => SUPPORTED_SUBSET_NAME_64BE_ROW,
        (WordSize::Bit32, Endianness::Little, CompressionMode::Binary) => {
            SUPPORTED_SUBSET_NAME_32LE_BINARY
        }
        (WordSize::Bit32, Endianness::Big, CompressionMode::Binary) => {
            SUPPORTED_SUBSET_NAME_32BE_BINARY
        }
        (WordSize::Bit64, Endianness::Little, CompressionMode::Binary) => {
            SUPPORTED_SUBSET_NAME_64LE_BINARY
        }
        (WordSize::Bit64, Endianness::Big, CompressionMode::Binary) => {
            SUPPORTED_SUBSET_NAME_64BE_BINARY
        }
        (WordSize::Bit32, Endianness::Little, CompressionMode::Unknown(_)) => {
            SUPPORTED_SUBSET_NAME_32LE_UNKNOWN
        }
        (WordSize::Bit32, Endianness::Big, CompressionMode::Unknown(_)) => {
            SUPPORTED_SUBSET_NAME_32BE_UNKNOWN
        }
        (WordSize::Bit64, Endianness::Little, CompressionMode::Unknown(_)) => {
            SUPPORTED_SUBSET_NAME_64LE_UNKNOWN
        }
        (WordSize::Bit64, Endianness::Big, CompressionMode::Unknown(_)) => {
            SUPPORTED_SUBSET_NAME_64BE_UNKNOWN
        }
    }
}

pub fn supported_subset(
    word_size: WordSize,
    endianness: Endianness,
    compression: CompressionMode,
) -> SupportedSubset {
    SupportedSubset {
        name: supported_subset_name(word_size, endianness, compression),
        word_size,
        endianness,
        compression,
    }
}

pub const SUPPORTED_SUBSET: SupportedSubset = SupportedSubset {
    name: SUPPORTED_SUBSET_NAME,
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
    Date,
    Time,
    DateTime,
    Duration,
}

impl SemanticTypeHint {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime",
            Self::Duration => "duration",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColumnMetadata {
    pub label: Option<String>,
    pub format_name: Option<String>,
    pub informat_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasMissingTag {
    Dot,
    Underscore,
    Letter(char),
}

impl SasMissingTag {
    pub fn code(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Underscore => '_',
            Self::Letter(tag) => *tag,
        }
    }

    pub fn from_code(tag: char) -> Option<Self> {
        match tag {
            '.' => Some(Self::Dot),
            '_' => Some(Self::Underscore),
            'A'..='Z' => Some(Self::Letter(tag)),
            _ => None,
        }
    }
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
    Float64 {
        value: f64,
        raw_bits: u64,
        missing_tag: Option<SasMissingTag>,
    },
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
            Self::Float64 { value, .. } => Some(*value),
            Self::DeferredBytes { .. } => None,
        }
    }

    pub fn raw_bits(&self) -> Option<u64> {
        match self {
            Self::Float64 { raw_bits, .. } => Some(*raw_bits),
            Self::DeferredBytes { .. } => None,
        }
    }

    pub fn width_bytes(&self) -> usize {
        match self {
            Self::Float64 { .. } => 8,
            Self::DeferredBytes { width_bytes, .. } => *width_bytes,
        }
    }

    pub fn raw_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Float64 { .. } => None,
            Self::DeferredBytes { raw_bytes, .. } => Some(raw_bytes.as_slice()),
        }
    }

    pub fn missing_tag(&self) -> Option<SasMissingTag> {
        match self {
            Self::Float64 { missing_tag, .. } => *missing_tag,
            Self::DeferredBytes { .. } => None,
        }
    }
}

impl From<f64> for NumericValue {
    fn from(value: f64) -> Self {
        Self::Float64 {
            value,
            raw_bits: value.to_bits(),
            missing_tag: None,
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubheaderRowRef {
    pub offset: usize,
    pub len: usize,
    pub compression: CompressionMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PageRowSource {
    pub page_index: usize,
    pub raw_data_offset: Option<usize>,
    pub raw_row_count: usize,
    pub subheader_rows: Vec<SubheaderRowRef>,
}

pub struct ParsedSas7bdat {
    pub metadata: SasMetadata,
    pub(crate) reader: BoxedParserDataSource,
    pub(crate) header_size: usize,
    pub(crate) row_sources: Vec<PageRowSource>,
    pub(crate) text_encoding_code: u8,
    pub(crate) next_row_source: usize,
    pub(crate) pending_rows: VecDeque<ParsedRow>,
    pub(crate) next_row_index: usize,
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
    pub(crate) fn new_streaming(
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
