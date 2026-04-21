use super::constants::{
    SAS7BDAT_ALIGNMENT_PADDING_OFFSET, SAS7BDAT_ENCODING_OFFSET, SAS7BDAT_ENDIANNESS_OFFSET,
    SAS7BDAT_HEADER_PREFIX_LEN, SAS7BDAT_HEADER_SIZE_BASE_OFFSET, SAS7BDAT_LAYOUT_FLAGS_32,
    SAS7BDAT_LAYOUT_FLAGS_64, SAS7BDAT_MAX_ALIGNMENT_PADDING_LEN, SAS7BDAT_PAGE_COUNT_BASE_OFFSET,
    SAS7BDAT_PAGE_SIZE_BASE_OFFSET, SAS7BDAT_TABLE_NAME_LEN, SAS7BDAT_TABLE_NAME_OFFSET,
    SAS7BDAT_WORD_SIZE_OFFSET,
};

#[derive(Debug, Clone, Copy)]
/// Header-prefix offsets used while decoding the SAS file header.
pub struct ParserOffsets {
    pub(crate) word_size: usize,
    pub(crate) endianness: usize,
    pub(crate) encoding: usize,
    pub(crate) table_name_start: usize,
    pub(crate) table_name_end: usize,
}

impl Default for ParserOffsets {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserOffsets {
    /// Build the default SAS7BDAT header-offset table.
    pub fn new() -> Self {
        Self {
            word_size: SAS7BDAT_WORD_SIZE_OFFSET,
            endianness: SAS7BDAT_ENDIANNESS_OFFSET,
            encoding: SAS7BDAT_ENCODING_OFFSET,
            table_name_start: SAS7BDAT_TABLE_NAME_OFFSET,
            table_name_end: SAS7BDAT_TABLE_NAME_OFFSET + SAS7BDAT_TABLE_NAME_LEN,
        }
    }

    /// Return the number of bytes read before fixed-width header decoding begins.
    pub fn header_prefix_len(self) -> usize {
        SAS7BDAT_HEADER_PREFIX_LEN
    }

    /// Return the header-size field offset for the current prefix.
    pub fn header_size_offset(self, header_prefix: &[u8]) -> usize {
        SAS7BDAT_HEADER_SIZE_BASE_OFFSET + self.alignment_padding_len(header_prefix)
    }

    /// Return the page-size field offset for the current prefix.
    pub fn page_size_offset(self, header_prefix: &[u8]) -> usize {
        SAS7BDAT_PAGE_SIZE_BASE_OFFSET + self.alignment_padding_len(header_prefix)
    }

    /// Return the page-count field offset for the current prefix.
    /// Return the page-count field offset for the current prefix.
    pub fn page_count_offset(self, header_prefix: &[u8]) -> usize {
        SAS7BDAT_PAGE_COUNT_BASE_OFFSET + self.alignment_padding_len(header_prefix)
    }

    fn alignment_padding_len(self, header_prefix: &[u8]) -> usize {
        match header_prefix
            .get(SAS7BDAT_ALIGNMENT_PADDING_OFFSET)
            .copied()
        {
            Some(SAS7BDAT_LAYOUT_FLAGS_64) => 4,
            Some(SAS7BDAT_LAYOUT_FLAGS_32) => 0,
            Some(value) if usize::from(value) <= SAS7BDAT_MAX_ALIGNMENT_PADDING_LEN => {
                usize::from(value)
            }
            _ => 0,
        }
    }
}
