use crate::parser::{
    Endianness, ParserError, WordSize,
    constants::{
        SAS7BDAT_BIG_ENDIAN_CODE, SAS7BDAT_LAYOUT_FLAGS_32, SAS7BDAT_LAYOUT_FLAGS_64,
        SAS7BDAT_LITTLE_ENDIAN_CODE,
    },
    layouts::{ColumnFormatLayout, PageHeaderLayout, RowSizeLayout, SubheaderLayout},
    read_u32, read_u64,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SasLayout {
    pub word_size: WordSize,
    pub endianness: Endianness,
}

impl SasLayout {
    pub const fn new(word_size: WordSize, endianness: Endianness) -> Self {
        Self {
            word_size,
            endianness,
        }
    }

    pub const fn bit64_little() -> Self {
        Self::new(WordSize::Bit64, Endianness::Little)
    }

    pub const fn bit64_big() -> Self {
        Self::new(WordSize::Bit64, Endianness::Big)
    }

    pub const fn bit32_little() -> Self {
        Self::new(WordSize::Bit32, Endianness::Little)
    }

    pub const fn word_size_from_marker(marker: u8) -> Option<WordSize> {
        match marker {
            SAS7BDAT_LAYOUT_FLAGS_32 => Some(WordSize::Bit32),
            SAS7BDAT_LAYOUT_FLAGS_64 => Some(WordSize::Bit64),
            _ => None,
        }
    }

    pub const fn endianness_from_marker(marker: u8) -> Option<Endianness> {
        const SAS_ENDIAN_LITTLE: u8 = SAS7BDAT_LITTLE_ENDIAN_CODE;
        const SAS_ENDIAN_BIG: u8 = SAS7BDAT_BIG_ENDIAN_CODE;

        match marker {
            SAS_ENDIAN_LITTLE => Some(Endianness::Little),
            SAS_ENDIAN_BIG => Some(Endianness::Big),
            _ => None,
        }
    }

    pub const fn from_markers(word_size_marker: u8, endianness_marker: u8) -> Option<Self> {
        let Some(word_size) = Self::word_size_from_marker(word_size_marker) else {
            return None;
        };
        let Some(endianness) = Self::endianness_from_marker(endianness_marker) else {
            return None;
        };
        Some(Self::new(word_size, endianness))
    }

    pub const fn word_size_marker(self) -> u8 {
        match self.word_size {
            WordSize::Bit32 => SAS7BDAT_LAYOUT_FLAGS_32,
            WordSize::Bit64 => SAS7BDAT_LAYOUT_FLAGS_64,
        }
    }

    pub const fn endianness_marker(self) -> u8 {
        match self.endianness {
            Endianness::Little => SAS7BDAT_LITTLE_ENDIAN_CODE,
            Endianness::Big => SAS7BDAT_BIG_ENDIAN_CODE,
        }
    }

    pub const fn word_size_bytes(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 4,
            WordSize::Bit64 => 8,
        }
    }

    pub const fn page_header_layout(self) -> PageHeaderLayout {
        match self.word_size {
            WordSize::Bit32 => PageHeaderLayout {
                size: 24,
                page_type_offset: 16,
                block_count_offset: 18,
                subheader_count_offset: 20,
                page_subheader_count_offset: 22,
            },
            WordSize::Bit64 => PageHeaderLayout {
                size: 40,
                page_type_offset: 32,
                block_count_offset: 34,
                subheader_count_offset: 36,
                page_subheader_count_offset: 38,
            },
        }
    }

    pub const fn page_header_size(self) -> usize {
        self.page_header_layout().size
    }

    pub const fn subheader_layout(self) -> SubheaderLayout {
        match self.word_size {
            WordSize::Bit32 => SubheaderLayout {
                pointer_size: 12,
                data_offset: 4,
                signature_size: 4,
                column_attrs_entry_size: 12,
            },
            WordSize::Bit64 => SubheaderLayout {
                pointer_size: 24,
                data_offset: 8,
                signature_size: 8,
                column_attrs_entry_size: 16,
            },
        }
    }

    pub const fn subheader_pointer_size(self) -> usize {
        self.subheader_layout().pointer_size
    }

    pub const fn subheader_data_offset(self) -> usize {
        self.subheader_layout().data_offset
    }

    pub const fn column_attrs_entry_size(self) -> usize {
        self.subheader_layout().column_attrs_entry_size
    }

    pub const fn subheader_signature_size(self) -> usize {
        self.subheader_layout().signature_size
    }

    pub const fn row_size_layout(self) -> RowSizeLayout {
        match self.word_size {
            WordSize::Bit32 => RowSizeLayout {
                row_length: 20,
                row_count: 24,
                column_count: 36,
                page_row_count: 60,
                page_size: 52,
            },
            WordSize::Bit64 => RowSizeLayout {
                row_length: 40,
                row_count: 48,
                column_count: 72,
                page_row_count: 120,
                page_size: 104,
            },
        }
    }

    pub const fn row_size_min_len(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 190,
            WordSize::Bit64 => 250,
        }
    }

    pub const fn column_format_layout(self) -> ColumnFormatLayout {
        match self.word_size {
            WordSize::Bit32 => ColumnFormatLayout {
                min_len: 46,
                format_width_offset: None,
                format_digits_offset: None,
                format_ref_offset: 34,
                label_ref_offset: 40,
            },
            WordSize::Bit64 => ColumnFormatLayout {
                min_len: 58,
                format_width_offset: Some(24),
                format_digits_offset: Some(26),
                format_ref_offset: 46,
                label_ref_offset: 52,
            },
        }
    }

    pub(crate) fn read_word(self, bytes: &[u8], offset: usize) -> Result<u64, ParserError> {
        match self.word_size {
            WordSize::Bit32 => Ok(read_u32(bytes, offset, self.endianness)? as u64),
            WordSize::Bit64 => read_u64(bytes, offset, self.endianness),
        }
    }

    pub fn numeric_bytes(self, value: f64) -> [u8; 8] {
        match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        }
    }
}

impl Default for SasLayout {
    fn default() -> Self {
        Self::bit64_little()
    }
}
