use crate::parser::constants::SAS7BDAT_MAGIC_NUMBER;

use super::offsets::ParserOffsets;
use super::{ParserError, SasLayout, decode_text_bytes, read_u32};

/// Decoded fields from the fixed SAS file header prefix.
/// Decoded fields from the fixed SAS file header prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasFileHeader {
    /// Declared word-size and endianness layout.
    /// Declared word-size and endianness layout.
    pub layout: SasLayout,
    /// Source text encoding code stored in the header.
    /// Source text encoding code stored in the header.
    pub text_encoding_code: u8,
    /// Header length in bytes.
    /// Header length in bytes.
    pub header_size: usize,
    /// Page size in bytes.
    /// Page size in bytes.
    pub page_size: usize,
    /// Number of pages declared by the file header.
    /// Number of pages declared by the file header.
    pub page_count: usize,
    /// Table name stored in the header.
    /// Table name stored in the header.
    pub table_name: String,
}

impl SasFileHeader {
    /// Decode a `SasFileHeader` from the raw header prefix bytes.
    pub fn from_header_prefix(
        header_prefix: &[u8],
        offsets: &ParserOffsets,
    ) -> Result<Self, ParserError> {
        let magic =
            header_prefix
                .get(..SAS7BDAT_MAGIC_NUMBER.len())
                .ok_or(ParserError::InvalidFormat(
                    "sas7bdat header prefix is truncated",
                ))?;
        if magic != SAS7BDAT_MAGIC_NUMBER {
            return Err(ParserError::InvalidFormat("missing sas7bdat magic number"));
        }

        let word_size_marker =
            *header_prefix
                .get(offsets.word_size)
                .ok_or(ParserError::InvalidFormat(
                    "sas7bdat header prefix is truncated",
                ))?;
        let endianness_marker =
            *header_prefix
                .get(offsets.endianness)
                .ok_or(ParserError::InvalidFormat(
                    "sas7bdat header prefix is truncated",
                ))?;
        let layout = SasLayout::from_markers(word_size_marker, endianness_marker).ok_or(
            if SasLayout::word_size_from_marker(word_size_marker).is_none() {
                ParserError::InvalidFormat("invalid sas7bdat word-size flag")
            } else {
                ParserError::InvalidFormat("invalid sas7bdat endianness flag")
            },
        )?;

        let text_encoding_code =
            *header_prefix
                .get(offsets.encoding)
                .ok_or(ParserError::InvalidFormat(
                    "sas7bdat header prefix is truncated",
                ))?;
        let table_name = decode_text_bytes(
            header_prefix
                .get(offsets.table_name_start..offsets.table_name_end)
                .ok_or(ParserError::InvalidFormat(
                    "sas7bdat header prefix is truncated",
                ))?,
            text_encoding_code,
        )?;
        let header_size = read_u32(
            header_prefix,
            offsets.header_size_offset(header_prefix),
            layout.endianness,
        )? as usize;
        let page_size = read_u32(
            header_prefix,
            offsets.page_size_offset(header_prefix),
            layout.endianness,
        )? as usize;
        let page_count =
            layout.read_word(header_prefix, offsets.page_count_offset(header_prefix))? as usize;

        if header_size < 1024 || page_size < 1024 {
            return Err(ParserError::InvalidFormat(
                "header_size and page_size must both be at least 1024 bytes",
            ));
        }

        Ok(Self {
            layout,
            text_encoding_code,
            header_size,
            page_size,
            page_count,
            table_name,
        })
    }

    /// Validate that the underlying file length can hold the declared pages.
    /// Validate that the underlying file length can hold the declared pages.
    pub fn validate_file_len(&self, file_len: usize) -> Result<(), ParserError> {
        let expected_len = self
            .header_size
            .checked_add(self.page_size.saturating_mul(self.page_count))
            .ok_or(ParserError::InvalidFormat("sas7bdat file size overflowed"))?;
        if file_len < expected_len {
            return Err(ParserError::InvalidFormat("sas7bdat file is truncated"));
        }
        Ok(())
    }
}
