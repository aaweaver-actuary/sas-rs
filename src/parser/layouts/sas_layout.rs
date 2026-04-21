use crate::parser::{
    Endianness, ParserError, WordSize,
    constants::{
        SAS7BDAT_BIG_ENDIAN_CODE, SAS7BDAT_LAYOUT_FLAGS_32, SAS7BDAT_LAYOUT_FLAGS_64,
        SAS7BDAT_LITTLE_ENDIAN_CODE,
    },
    layouts::{ColumnFormatLayout, PageHeaderLayout, RowSizeLayout, SubheaderLayout},
    read_u32, read_u64,
};

/// Word-size and endianness layout for a SAS7BDAT file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SasLayout {
    /// Declared word size for pointer-sized fields in the file.
    pub word_size: WordSize,
    /// Declared byte order for integer and floating-point fields.
    pub endianness: Endianness,
}

impl SasLayout {
    /// Build a layout from explicit word-size and endianness markers.
    pub const fn new(word_size: WordSize, endianness: Endianness) -> Self {
        Self {
            word_size,
            endianness,
        }
    }

    /// Return the canonical 64-bit little-endian layout used by the primary subset.
    pub const fn bit64_little() -> Self {
        Self::new(WordSize::Bit64, Endianness::Little)
    }

    /// Return the 64-bit big-endian layout.
    pub const fn bit64_big() -> Self {
        Self::new(WordSize::Bit64, Endianness::Big)
    }

    /// Return the 32-bit little-endian layout.
    pub const fn bit32_little() -> Self {
        Self::new(WordSize::Bit32, Endianness::Little)
    }

    /// Decode the SAS word-size marker into a `WordSize`.
    pub const fn word_size_from_marker(marker: u8) -> Option<WordSize> {
        match marker {
            SAS7BDAT_LAYOUT_FLAGS_32 => Some(WordSize::Bit32),
            SAS7BDAT_LAYOUT_FLAGS_64 => Some(WordSize::Bit64),
            _ => None,
        }
    }

    /// Decode the SAS endianness marker into an `Endianness`.
    pub const fn endianness_from_marker(marker: u8) -> Option<Endianness> {
        const SAS_ENDIAN_LITTLE: u8 = SAS7BDAT_LITTLE_ENDIAN_CODE;
        const SAS_ENDIAN_BIG: u8 = SAS7BDAT_BIG_ENDIAN_CODE;

        match marker {
            SAS_ENDIAN_LITTLE => Some(Endianness::Little),
            SAS_ENDIAN_BIG => Some(Endianness::Big),
            _ => None,
        }
    }

    /// Build a layout from the raw SAS header markers.
    pub const fn from_markers(word_size_marker: u8, endianness_marker: u8) -> Option<Self> {
        let Some(word_size) = Self::word_size_from_marker(word_size_marker) else {
            return None;
        };
        let Some(endianness) = Self::endianness_from_marker(endianness_marker) else {
            return None;
        };
        Some(Self::new(word_size, endianness))
    }

    /// Encode this layout as the SAS word-size marker.
    pub const fn word_size_marker(self) -> u8 {
        match self.word_size {
            WordSize::Bit32 => SAS7BDAT_LAYOUT_FLAGS_32,
            WordSize::Bit64 => SAS7BDAT_LAYOUT_FLAGS_64,
        }
    }

    /// Encode this layout as the SAS endianness marker.
    pub const fn endianness_marker(self) -> u8 {
        match self.endianness {
            Endianness::Little => SAS7BDAT_LITTLE_ENDIAN_CODE,
            Endianness::Big => SAS7BDAT_BIG_ENDIAN_CODE,
        }
    }

    /// Return the pointer width in bytes for this layout.
    pub const fn word_size_bytes(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 4,
            WordSize::Bit64 => 8,
        }
    }

    /// Return the page-header field offsets for this layout.
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

    /// Return the page-header length in bytes for this layout.
    pub const fn page_header_size(self) -> usize {
        self.page_header_layout().size
    }

    /// Return the subheader pointer layout for this file layout.
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

    /// Return the byte width of one subheader pointer entry.
    pub const fn subheader_pointer_size(self) -> usize {
        self.subheader_layout().pointer_size
    }

    /// Return the offset from a pointer entry to the subheader payload.
    pub const fn subheader_data_offset(self) -> usize {
        self.subheader_layout().data_offset
    }

    /// Return the byte width of one column-attributes entry.
    pub const fn column_attrs_entry_size(self) -> usize {
        self.subheader_layout().column_attrs_entry_size
    }

    /// Return the byte width of a raw subheader signature field.
    pub const fn subheader_signature_size(self) -> usize {
        self.subheader_layout().signature_size
    }

    /// Return the offsets used by the row-size subheader under this layout.
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

    /// Return the minimum byte length accepted for the row-size subheader.
    pub const fn row_size_min_len(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 190,
            WordSize::Bit64 => 250,
        }
    }

    /// Return the offsets used by the column-format subheader under this layout.
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

    /// Encode a numeric value using the layout endianness.
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
