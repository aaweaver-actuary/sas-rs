pub mod column_metadata;
pub mod compression_mode;
pub mod endianness;
pub mod numeric_value;
pub mod parsed_row;
pub mod parsed_sas7bdat;
pub mod parsed_value;
pub mod parser_data_source;
pub mod parser_input;
pub mod row_batch;
pub mod sas_column;
pub mod sas_metadata;
pub mod sas_missing_tag;
pub mod semantic_type_hint;
pub mod supported_subset_const;
pub mod supported_subset_fn;
pub mod supported_subset_name_const;
pub mod supported_subset_type;

pub use super::UnsupportedFeature;

use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedSubset {
    pub name: &'static str,
    pub word_size: WordSize,
    pub endianness: Endianness,
    pub compression: CompressionMode,
}

pub const SUPPORTED_SUBSET_NAME: &str = "sas7bdat-64le-uncompressed-v1";

const UNCOMPRESSED_SUPPORTED_SUBSET_NAMES: [[&str; 2]; 2] = [
    [
        "sas7bdat-32le-uncompressed-v1",
        "sas7bdat-32be-uncompressed-v1",
    ],
    [SUPPORTED_SUBSET_NAME, "sas7bdat-64be-uncompressed-v1"],
];
const ROW_COMPRESSED_SUPPORTED_SUBSET_NAMES: [[&str; 2]; 2] = [
    [
        "sas7bdat-32le-row-compressed-v1",
        "sas7bdat-32be-row-compressed-v1",
    ],
    [
        "sas7bdat-64le-row-compressed-v1",
        "sas7bdat-64be-row-compressed-v1",
    ],
];
const BINARY_COMPRESSED_SUPPORTED_SUBSET_NAMES: [[&str; 2]; 2] = [
    [
        "sas7bdat-32le-binary-compressed-v1",
        "sas7bdat-32be-binary-compressed-v1",
    ],
    [
        "sas7bdat-64le-binary-compressed-v1",
        "sas7bdat-64be-binary-compressed-v1",
    ],
];
const UNKNOWN_COMPRESSION_SUPPORTED_SUBSET_NAMES: [[&str; 2]; 2] = [
    [
        "sas7bdat-32le-unknown-compression-v1",
        "sas7bdat-32be-unknown-compression-v1",
    ],
    [
        "sas7bdat-64le-unknown-compression-v1",
        "sas7bdat-64be-unknown-compression-v1",
    ],
];

fn supported_subset_name(
    word_size: WordSize,
    endianness: Endianness,
    compression: CompressionMode,
) -> &'static str {
    let word_size_index = match word_size {
        WordSize::Bit32 => 0,
        WordSize::Bit64 => 1,
    };
    let endianness_index = match endianness {
        Endianness::Little => 0,
        Endianness::Big => 1,
    };
    let table = match compression {
        CompressionMode::None => UNCOMPRESSED_SUPPORTED_SUBSET_NAMES,
        CompressionMode::Row => ROW_COMPRESSED_SUPPORTED_SUBSET_NAMES,
        CompressionMode::Binary => BINARY_COMPRESSED_SUPPORTED_SUBSET_NAMES,
        CompressionMode::Unknown(_) => UNKNOWN_COMPRESSION_SUPPORTED_SUBSET_NAMES,
    };
    table[word_size_index][endianness_index]
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
