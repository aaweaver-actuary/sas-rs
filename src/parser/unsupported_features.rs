use std::fmt::{self, Display, Formatter};

use super::contracts::{CompressionMode, Endianness, WordSize};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Feature classifications for inputs outside the supported parser subset.
pub enum UnsupportedFeature {
    /// Unsupported word size.
    WordSize(WordSize),
    /// Unsupported endianness.
    Endianness(Endianness),
    /// Unsupported compression mode.
    Compression(CompressionMode),
    /// Unsupported text encoding code.
    Encoding(u8),
    /// Unsupported page type.
    PageType(u16),
    /// Unsupported subheader signature.
    SubheaderSignature(u32),
    /// Unsupported numeric cell width.
    NumericWidth(u32),
    /// Unsupported column type code.
    ColumnType(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parser failures returned by the public parsing API.
pub enum ParserError {
    /// The input violated the supported SAS7BDAT structure.
    InvalidFormat(&'static str),
    /// The input used a feature outside the supported subset.
    Unsupported(UnsupportedFeature),
    /// An I/O failure occurred while reading the source.
    Io(String),
}

impl Display for UnsupportedFeature {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::WordSize(WordSize::Bit32) => {
                formatter.write_str("32-bit layout is outside the supported subset")
            }
            Self::WordSize(WordSize::Bit64) => {
                formatter.write_str("64-bit layout is already supported")
            }
            Self::Endianness(Endianness::Big) => {
                formatter.write_str("big-endian files are outside the supported subset")
            }
            Self::Endianness(Endianness::Little) => {
                formatter.write_str("little-endian files are already supported")
            }
            Self::Compression(mode) => write!(
                formatter,
                "compression {:?} is outside the supported subset",
                mode
            ),
            Self::Encoding(code) => write!(
                formatter,
                "encoding code {} is outside the supported subset",
                code
            ),
            Self::PageType(page_type) => write!(
                formatter,
                "page type 0x{page_type:04X} is outside the supported subset"
            ),
            Self::SubheaderSignature(signature) => write!(
                formatter,
                "subheader signature 0x{signature:08X} is outside the supported subset"
            ),
            Self::NumericWidth(width) => write!(
                formatter,
                "numeric width {} is outside the supported subset",
                width
            ),
            Self::ColumnType(code) => write!(
                formatter,
                "column type code {} is outside the supported subset",
                code
            ),
        }
    }
}
