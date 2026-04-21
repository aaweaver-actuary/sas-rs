/// Re-export of optional SAS column display metadata.
pub mod column_metadata;
/// Re-export of supported SAS compression modes.
pub mod compression_mode;
/// Re-export of SAS endianness markers.
pub mod endianness;
/// Re-export of decoded or deferred numeric cell values.
pub mod numeric_value;
/// Re-export of decoded logical rows.
pub mod parsed_row;
/// Re-export of the streaming parsed dataset handle.
pub mod parsed_sas7bdat;
/// Re-export of decoded logical cell values.
pub mod parsed_value;
/// Re-export of the boxed parser data-source trait path.
pub mod parser_data_source;
/// Re-export of the public parser input wrapper.
pub mod parser_input;
/// Re-export of decoded row batches.
pub mod row_batch;
/// Re-export of decoded row-batch column descriptors.
pub mod row_batch_column;
/// Re-export of decoded row-batch schemas.
pub mod row_batch_schema;
/// Re-export of decoded logical row value kinds.
pub mod row_value_kind;
/// Re-export of parsed SAS source-column metadata.
pub mod sas_column;
/// Re-export of parsed dataset-level metadata.
pub mod sas_metadata;
/// Re-export of SAS special-missing tags.
pub mod sas_missing_tag;
/// Re-export of inferred SAS semantic type hints.
pub mod semantic_type_hint;
/// Re-export of the default supported-subset constant path.
pub mod supported_subset_const;
/// Re-export of the supported-subset constructor path.
pub mod supported_subset_fn;
/// Re-export of the default supported-subset name constant path.
pub mod supported_subset_name_const;
/// Re-export of the supported-subset type path.
pub mod supported_subset_type;

pub use super::types::BoxedParserDataSource;
pub use super::unsupported_features::UnsupportedFeature;
pub use column_metadata::ColumnMetadata;
pub use compression_mode::CompressionMode;
pub use endianness::Endianness;
pub use numeric_value::NumericValue;
pub use parsed_row::ParsedRow;
pub use parsed_sas7bdat::ParsedSas7bdat;
pub use parsed_value::ParsedValue;
pub use parser_data_source::ParserDataSource;
pub use parser_input::ParserInput;
pub use row_batch::RowBatch;
pub use row_batch_column::RowBatchColumn;
pub use row_batch_schema::RowBatchSchema;
pub use row_value_kind::RowValueKind;
pub use sas_column::SasColumn;
pub use sas_metadata::SasMetadata;
pub use sas_missing_tag::SasMissingTag;
pub use semantic_type_hint::SemanticTypeHint;

/// Supported SAS word size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordSize {
    /// 32-bit pointer layout.
    Bit32,
    /// 64-bit pointer layout.
    Bit64,
}

/// Physical SAS column kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnKind {
    /// Numeric column.
    Numeric,
    /// Character column.
    String,
}

/// Stable identifier for a parser subset the crate understands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedSubset {
    /// Stable machine-readable subset name.
    pub name: &'static str,
    /// Supported word size.
    pub word_size: WordSize,
    /// Supported byte order.
    pub endianness: Endianness,
    /// Supported compression mode.
    pub compression: CompressionMode,
}

/// Stable name for the default supported subset.
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

/// Build a supported-subset descriptor from its physical attributes.
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

/// Default supported subset for the public parser entrypoint.
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
