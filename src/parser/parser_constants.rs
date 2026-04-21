use super::constants::{
    SAS7BDAT_AMD_PAGE_TYPE_CODE, SAS7BDAT_BIG_ENDIAN_CODE,
    SAS7BDAT_COLUMN_ATTRS_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_FORMAT_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_COLUMN_NAME_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_SIZE_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_COLUMN_TEXT_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_TYPE_CHR, SAS7BDAT_COLUMN_TYPE_NUM, SAS7BDAT_COMPRESSION_SIGNATURE_RDC,
    SAS7BDAT_COMPRESSION_SIGNATURE_RLE, SAS7BDAT_COUNTS_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_DATA_PAGE_TYPE_CODE, SAS7BDAT_LAYOUT_FLAGS_32, SAS7BDAT_LAYOUT_FLAGS_64,
    SAS7BDAT_LITTLE_ENDIAN_CODE, SAS7BDAT_MAGIC_NUMBER, SAS7BDAT_META_PAGE_TYPE_CODE,
    SAS7BDAT_MIX_PAGE_TYPE_CODE, SAS7BDAT_ROW_SIZE_SUBHEADER_SIGNATURE_CODE,
};

/// Raw layout flag bytes used in SAS headers.
/// Raw layout flag bytes used in SAS headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutFlags {
    /// Marker byte for 32-bit layouts.
    /// Marker byte for 32-bit layouts.
    pub bit32: u8,
    /// Marker byte for 64-bit layouts.
    /// Marker byte for 64-bit layouts.
    pub bit64: u8,
}

/// Raw endianness flag bytes used in SAS headers.
/// Raw endianness flag bytes used in SAS headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndiannessFlags {
    /// Marker byte for big-endian files.
    /// Marker byte for big-endian files.
    pub big: u8,
    /// Marker byte for little-endian files.
    /// Marker byte for little-endian files.
    pub little: u8,
}

/// Page-type discriminator codes recognized by the parser.
/// Page-type discriminator codes recognized by the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTypeConstants {
    /// Code for metadata pages.
    /// Code for metadata pages.
    pub meta: u16,
    /// Code for data pages.
    /// Code for data pages.
    pub data: u16,
    /// Code for mixed metadata/data pages.
    /// Code for mixed metadata/data pages.
    pub mix: u16,
    /// Code for AMD pages.
    /// Code for AMD pages.
    pub amd: u16,
}

/// Known subheader signature codes used during metadata parsing.
/// Known subheader signature codes used during metadata parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubheaderSignatureConstants {
    /// Signature for the row-size subheader.
    /// Signature for the row-size subheader.
    pub row_size: u32,
    /// Signature for the column-size subheader.
    /// Signature for the column-size subheader.
    pub column_size: u32,
    /// Signature for the counts subheader.
    /// Signature for the counts subheader.
    pub counts: u32,
    /// Signature for the column-format subheader.
    /// Signature for the column-format subheader.
    pub column_format: u32,
    /// Signature for the column-attributes subheader.
    /// Signature for the column-attributes subheader.
    pub column_attrs: u32,
    /// Signature for the column-text subheader.
    /// Signature for the column-text subheader.
    pub column_text: u32,
    /// Signature for the column-name subheader.
    /// Signature for the column-name subheader.
    pub column_name: u32,
}

/// Raw column-type codes recognized by the parser.
/// Raw column-type codes recognized by the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnTypeConstants {
    /// Code for numeric columns.
    /// Code for numeric columns.
    pub numeric: u8,
    /// Code for character columns.
    /// Code for character columns.
    pub string: u8,
}

/// Compression signature strings recognized in metadata.
/// Compression signature strings recognized in metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionSignatureConstants {
    /// Signature for row compression.
    /// Signature for row compression.
    pub row: &'static str,
    /// Signature for binary compression.
    /// Signature for binary compression.
    pub binary: &'static str,
}

/// Shared raw constants used by parser tests and fixture builders.
/// Shared raw constants used by parser tests and fixture builders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserConstants {
    /// Expected 32-byte SAS file magic header.
    /// Expected 32-byte SAS file magic header.
    pub magic_number: [u8; 32],
    /// Layout flag bytes used by the format.
    /// Layout flag bytes used by the format.
    pub layout_flags: LayoutFlags,
    /// Endianness flag bytes used by the format.
    /// Endianness flag bytes used by the format.
    pub endianness_flags: EndiannessFlags,
    /// Known page-type codes.
    /// Known page-type codes.
    pub page_types: PageTypeConstants,
    /// Known subheader signature codes.
    /// Known subheader signature codes.
    pub subheader_signatures: SubheaderSignatureConstants,
    /// Known column-type codes.
    /// Known column-type codes.
    pub column_types: ColumnTypeConstants,
    /// Known compression signature strings.
    /// Known compression signature strings.
    pub compression_signatures: CompressionSignatureConstants,
}

impl ParserConstants {
    /// Return the shared parser constant set used across the crate.
    pub const fn shared() -> Self {
        Self {
            magic_number: SAS7BDAT_MAGIC_NUMBER,
            layout_flags: LayoutFlags {
                bit32: SAS7BDAT_LAYOUT_FLAGS_32,
                bit64: SAS7BDAT_LAYOUT_FLAGS_64,
            },
            endianness_flags: EndiannessFlags {
                big: SAS7BDAT_BIG_ENDIAN_CODE,
                little: SAS7BDAT_LITTLE_ENDIAN_CODE,
            },
            page_types: PageTypeConstants {
                meta: SAS7BDAT_META_PAGE_TYPE_CODE,
                data: SAS7BDAT_DATA_PAGE_TYPE_CODE,
                mix: SAS7BDAT_MIX_PAGE_TYPE_CODE,
                amd: SAS7BDAT_AMD_PAGE_TYPE_CODE,
            },
            subheader_signatures: SubheaderSignatureConstants {
                row_size: SAS7BDAT_ROW_SIZE_SUBHEADER_SIGNATURE_CODE,
                column_size: SAS7BDAT_COLUMN_SIZE_SUBHEADER_SIGNATURE_CODE,
                counts: SAS7BDAT_COUNTS_SUBHEADER_SIGNATURE_CODE,
                column_format: SAS7BDAT_COLUMN_FORMAT_SUBHEADER_SIGNATURE_CODE,
                column_attrs: SAS7BDAT_COLUMN_ATTRS_SUBHEADER_SIGNATURE_CODE,
                column_text: SAS7BDAT_COLUMN_TEXT_SUBHEADER_SIGNATURE_CODE,
                column_name: SAS7BDAT_COLUMN_NAME_SUBHEADER_SIGNATURE_CODE,
            },
            column_types: ColumnTypeConstants {
                numeric: SAS7BDAT_COLUMN_TYPE_NUM,
                string: SAS7BDAT_COLUMN_TYPE_CHR,
            },
            compression_signatures: CompressionSignatureConstants {
                row: SAS7BDAT_COMPRESSION_SIGNATURE_RLE,
                binary: SAS7BDAT_COMPRESSION_SIGNATURE_RDC,
            },
        }
    }
}
