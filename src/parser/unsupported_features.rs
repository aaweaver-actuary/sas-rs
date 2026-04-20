
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsupportedFeature {
    WordSize(WordSize),
    Endianness(Endianness),
    Compression(CompressionMode),
    Encoding(u8),
    PageType(u16),
    SubheaderSignature(u32),
    NumericWidth(u32),
    ColumnType(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    InvalidFormat(&'static str),
    Unsupported(UnsupportedFeature),
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
