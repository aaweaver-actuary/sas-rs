use super::constants::SAS_ALIGNMENT_OFFSET_4;

const WORD_SIZE_OFFSET: usize = 32;
const ALIGNMENT_PADDING_OFFSET: usize = 35;
const ENDIANNESS_OFFSET: usize = 37;
const ENCODING_OFFSET: usize = 70;
const TABLE_NAME_OFFSET: usize = 92;
const TABLE_NAME_LEN: usize = 32;
const HEADER_SIZE_BASE_OFFSET: usize = 196;
const PAGE_SIZE_BASE_OFFSET: usize = 200;
const PAGE_COUNT_BASE_OFFSET: usize = 204;
const MAX_ALIGNMENT_PADDING_LEN: usize = 4;
const HEADER_PREFIX_LEN: usize = PAGE_COUNT_BASE_OFFSET + 8 + MAX_ALIGNMENT_PADDING_LEN;

#[derive(Debug)]
pub struct ParserOffsets {
    pub word_size: usize,
    pub alignment_padding: usize,
    pub endianness: usize,
    pub encoding: usize,
    pub table_name_start: usize,
    pub table_name_end: usize,
    pub header_size_base: usize,
    pub page_size_base: usize,
    pub page_count_base: usize,
}

impl Default for ParserOffsets {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserOffsets {
    pub fn new() -> Self {
        Self {
            word_size: WORD_SIZE_OFFSET,
            alignment_padding: ALIGNMENT_PADDING_OFFSET,
            endianness: ENDIANNESS_OFFSET,
            encoding: ENCODING_OFFSET,
            table_name_start: TABLE_NAME_OFFSET,
            table_name_end: TABLE_NAME_OFFSET + TABLE_NAME_LEN,
            header_size_base: HEADER_SIZE_BASE_OFFSET,
            page_size_base: PAGE_SIZE_BASE_OFFSET,
            page_count_base: PAGE_COUNT_BASE_OFFSET,
        }
    }

    pub fn header_prefix_len(&self) -> usize {
        HEADER_PREFIX_LEN
    }

    pub fn alignment_padding_len(&self, header_prefix: &[u8]) -> usize {
        if header_prefix.get(self.alignment_padding).copied() == Some(SAS_ALIGNMENT_OFFSET_4) {
            MAX_ALIGNMENT_PADDING_LEN
        } else {
            0
        }
    }

    pub fn header_size_offset(&self, header_prefix: &[u8]) -> usize {
        self.header_size_base + self.alignment_padding_len(header_prefix)
    }

    pub fn page_size_offset(&self, header_prefix: &[u8]) -> usize {
        self.page_size_base + self.alignment_padding_len(header_prefix)
    }

    pub fn page_count_offset(&self, header_prefix: &[u8]) -> usize {
        self.page_count_base + self.alignment_padding_len(header_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_offsets() {
        let offsets = ParserOffsets::new();
        assert_eq!(offsets.word_size, 32);
        assert_eq!(offsets.alignment_padding, 35);
        assert_eq!(offsets.endianness, 37);
        assert_eq!(offsets.encoding, 70);
        assert_eq!(offsets.table_name_start, 92);
        assert_eq!(offsets.table_name_end, 124);
        assert_eq!(offsets.header_size_base, 196);
        assert_eq!(offsets.page_size_base, 200);
        assert_eq!(offsets.page_count_base, 204);
        assert_eq!(offsets.header_prefix_len(), 216);
    }
}
