use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Seek, SeekFrom};

use encoding_rs::{Encoding, GB18030, ISO_8859_15, WINDOWS_1251, WINDOWS_1252};

pub mod column_type;
pub mod compression_signature;
pub mod constants;
pub mod contracts;
pub mod encoding;
pub mod endianness;
mod header;
pub mod layouts;
mod row;
pub mod sas_layout;
pub mod sas_page_type;
mod subheader;
pub mod types;
pub mod unsupported_features;

pub use contracts::{
    BoxedParserDataSource, ColumnKind, ColumnMetadata, CompressionMode, Endianness, NumericValue,
    ParsedRow, ParsedSas7bdat, ParsedValue, ParserDataSource, ParserInput, RowBatch,
    SUPPORTED_SUBSET, SasColumn, SasMetadata, SasMissingTag, SemanticTypeHint, SupportedSubset,
    WordSize,
};
pub use header::SasFileHeader;
pub use subheader::SasSubheaderSignature;

use self::contracts::{PageRowSource, SubheaderRowRef};
use self::row::{parse_row, parse_subheader_row};
use self::subheader::{SasMetadataAccumulator, parse_meta_page};

impl Display for ParserError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(message) => formatter.write_str(message),
            Self::Unsupported(feature) => feature.fmt(formatter),
            Self::Io(message) => formatter.write_str(message),
        }
    }
}

impl Error for ParserError {}

pub trait Sas7bdatParser {
    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SupportedSas7bdatParser;

impl Sas7bdatParser for SupportedSas7bdatParser {
    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError> {
        let offsets = ParserOffsets::new();
        parse_supported_subset(input, &offsets)
    }
}

#[derive(Debug, Clone, Copy)]
enum TextEncoding {
    Utf8,
    Latin1,
    EncodingRs(&'static Encoding),
}

impl TextEncoding {
    fn from_code(code: u8) -> Option<Self> {
        match code {
            UTF8_ENCODING_CODE | 28 => Some(Self::Utf8),
            LATIN1_ENCODING_CODE => Some(Self::Latin1),
            40 => Some(Self::EncodingRs(ISO_8859_15)),
            61 => Some(Self::EncodingRs(WINDOWS_1251)),
            125 => Some(Self::EncodingRs(GB18030)),
            WINDOWS_1252_ENCODING_CODE | DEFAULT_ENCODING_CODE | 204 => {
                Some(Self::EncodingRs(WINDOWS_1252))
            }
            _ => None,
        }
    }

    fn decode(self, bytes: &[u8]) -> Result<String, ParserError> {
        let trimmed = trim_padded_bytes(bytes);
        match self {
            Self::Utf8 => std::str::from_utf8(trimmed)
                .map(|value| value.to_string())
                .map_err(|_| {
                    ParserError::InvalidFormat(
                        "text could not be decoded with the declared source encoding",
                    )
                }),
            Self::Latin1 => Ok(trimmed.iter().map(|byte| char::from(*byte)).collect()),
            Self::EncodingRs(encoding) => {
                let (decoded, _, had_errors) = encoding.decode(trimmed);
                if had_errors {
                    return Err(ParserError::InvalidFormat(
                        "text could not be decoded with the declared source encoding",
                    ));
                }
                Ok(decoded.into_owned())
            }
        }
    }
}

fn parse_supported_subset(
    mut input: ParserInput<'_>,
    offsets: &ParserOffsets,
) -> Result<ParsedSas7bdat, ParserError> {
    let mut header_prefix = vec![0_u8; offsets.header_prefix_len()];
    input
        .reader
        .read_exact(&mut header_prefix)
        .map_err(io_error)?;

    let file_header = SasFileHeader::from_header_prefix(&header_prefix, offsets)?;
    let file_len = input.reader.seek(SeekFrom::End(0)).map_err(io_error)? as usize;
    file_header.validate_file_len(file_len)?;

    let mut metadata = SasMetadataAccumulator::new(
        file_header.table_name.clone(),
        file_header.page_size,
        file_header.page_count,
        file_header.text_encoding_code,
    );
    let mut row_sources = Vec::new();

    for page_index in 0..file_header.page_count {
        let page_header = SasPageHeader::read(
            input.reader.as_mut(),
            file_header.header_size,
            file_header.page_size,
            page_index,
            file_header.layout,
        )?;
        if page_header.uses_compressed_storage {
            row_sources.push(PageRowSource {
                page_index,
                raw_data_offset: Some(file_header.layout.page_header_size()),
                raw_row_count: page_header.raw_row_count,
                subheader_rows: Vec::new(),
            });
            continue;
        }

        match page_header.kind {
            Meta | Mix | Amd => {
                let page = read_page(
                    input.reader.as_mut(),
                    file_header.header_size,
                    file_header.page_size,
                    page_index,
                )?;
                let parsed_page = parse_meta_page(
                    &page,
                    &mut metadata,
                    file_header.layout,
                    matches!(page_header.kind, Mix),
                )?;
                let raw_row_count = if matches!(page_header.kind, Mix) {
                    metadata.page_row_count()
                } else {
                    0
                };
                if !parsed_page.subheader_rows.is_empty() || parsed_page.raw_data_offset.is_some() {
                    row_sources.push(PageRowSource {
                        page_index,
                        raw_data_offset: parsed_page.raw_data_offset,
                        raw_row_count,
                        subheader_rows: parsed_page.subheader_rows,
                    });
                }
            }
            Data => {
                row_sources.push(PageRowSource {
                    page_index,
                    raw_data_offset: Some(file_header.layout.page_header_size()),
                    raw_row_count: page_header.raw_row_count,
                    subheader_rows: Vec::new(),
                });
            }
            SasPageKind::Unknown(other) => {
                return Err(ParserError::Unsupported(UnsupportedFeature::PageType(
                    other,
                )));
            }
        }
    }

    if matches!(metadata.compression(), CompressionMode::Unknown(_)) {
        return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
            metadata.compression(),
        )));
    }

    let dataset_metadata = metadata.into_metadata(file_header.layout)?;

    Ok(ParsedSas7bdat::new_streaming(
        dataset_metadata,
        input.reader,
        file_header.header_size,
        row_sources,
        file_header.text_encoding_code,
    ))
}

impl ParsedSas7bdat {
    pub fn next_batch(&mut self, batch_size_rows: usize) -> Result<Option<RowBatch>, ParserError> {
        if batch_size_rows == 0 {
            return Err(ParserError::InvalidFormat(
                "batch_size_rows must be greater than zero",
            ));
        }

        self.fill_pending_rows(batch_size_rows)?;
        if self.pending_rows.is_empty() {
            return Ok(None);
        }

        let row_index_start = self.next_row_index;
        let take_count = self.pending_rows.len().min(batch_size_rows);
        let rows = self.pending_rows.drain(..take_count).collect::<Vec<_>>();
        self.next_row_index += rows.len();

        Ok(Some(RowBatch {
            row_index_start,
            rows,
        }))
    }

    fn fill_pending_rows(&mut self, min_rows: usize) -> Result<(), ParserError> {
        while self.pending_rows.len() < min_rows
            && self.next_row_source < self.row_sources.len()
            && self.decoded_row_count() < self.metadata.row_count
        {
            self.load_next_row_source()?;
        }

        if self.pending_rows.is_empty()
            && self.next_row_source == self.row_sources.len()
            && self.decoded_row_count() != self.metadata.row_count
        {
            return Err(ParserError::InvalidFormat(
                "row sources did not yield the expected row count",
            ));
        }

        Ok(())
    }

    fn load_next_row_source(&mut self) -> Result<(), ParserError> {
        let source = self.row_sources.get(self.next_row_source).cloned().ok_or(
            ParserError::InvalidFormat("row source index is out of bounds"),
        )?;
        self.next_row_source += 1;

        let page = read_page(
            self.reader.as_mut(),
            self.header_size,
            self.metadata.page_size,
            source.page_index,
        )?;

        for subheader_row in &source.subheader_rows {
            if self.decoded_row_count() == self.metadata.row_count {
                return Ok(());
            }
            let row = parse_subheader_row(
                &page,
                subheader_row.offset,
                subheader_row.len,
                subheader_row.compression,
                self.metadata.row_length,
            )?;
            self.pending_rows
                .push_back(parse_row(&row, &self.metadata, self.text_encoding_code)?);
        }

        if let Some(raw_data_offset) = source.raw_data_offset {
            let data = page
                .get(raw_data_offset..)
                .ok_or(ParserError::InvalidFormat(
                    "raw row data offset is outside the page",
                ))?;
            for row_index in 0..source.raw_row_count {
                if self.decoded_row_count() == self.metadata.row_count {
                    break;
                }

                let start = row_index
                    .checked_mul(self.metadata.row_length)
                    .ok_or(ParserError::InvalidFormat("row offset overflowed"))?;
                let end = start
                    .checked_add(self.metadata.row_length)
                    .ok_or(ParserError::InvalidFormat("row end offset overflowed"))?;
                let row = data
                    .get(start..end)
                    .ok_or(ParserError::InvalidFormat("row source row is truncated"))?;
                self.pending_rows.push_back(parse_row(
                    row,
                    &self.metadata,
                    self.text_encoding_code,
                )?);
            }
        }

        Ok(())
    }

    fn decoded_row_count(&self) -> usize {
        self.next_row_index + self.pending_rows.len()
    }
}

pub(crate) fn read_page_header(
    reader: &mut dyn ParserDataSource,
    header_size: usize,
    page_size: usize,
    page_index: usize,
    header: &mut [u8],
) -> Result<(), ParserError> {
    let offset = page_offset(header_size, page_size, page_index)?;
    read_exact_at(reader, offset, header)
}

pub(crate) fn read_page(
    reader: &mut dyn ParserDataSource,
    header_size: usize,
    page_size: usize,
    page_index: usize,
) -> Result<Vec<u8>, ParserError> {
    let offset = page_offset(header_size, page_size, page_index)?;
    let mut page = vec![0_u8; page_size];
    read_exact_at(reader, offset, &mut page)?;
    Ok(page)
}

fn page_offset(
    header_size: usize,
    page_size: usize,
    page_index: usize,
) -> Result<usize, ParserError> {
    header_size
        .checked_add(page_index.saturating_mul(page_size))
        .ok_or(ParserError::InvalidFormat("page offset overflowed"))
}

fn read_exact_at(
    reader: &mut dyn ParserDataSource,
    offset: usize,
    buffer: &mut [u8],
) -> Result<(), ParserError> {
    let offset =
        u64::try_from(offset).map_err(|_| ParserError::InvalidFormat("page offset overflowed"))?;
    reader.seek(SeekFrom::Start(offset)).map_err(io_error)?;
    reader.read_exact(buffer).map_err(io_error)
}

pub(crate) fn read_u16(
    bytes: &[u8],
    offset: usize,
    endianness: Endianness,
) -> Result<u16, ParserError> {
    let raw = bytes
        .get(offset..offset + 2)
        .ok_or(ParserError::InvalidFormat("expected a 16-bit value"))?;
    Ok(match endianness {
        Endianness::Little => u16::from_le_bytes([raw[0], raw[1]]),
        Endianness::Big => u16::from_be_bytes([raw[0], raw[1]]),
    })
}

pub(crate) fn read_u32(
    bytes: &[u8],
    offset: usize,
    endianness: Endianness,
) -> Result<u32, ParserError> {
    let raw = bytes
        .get(offset..offset + 4)
        .ok_or(ParserError::InvalidFormat("expected a 32-bit value"))?;
    Ok(match endianness {
        Endianness::Little => u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]),
        Endianness::Big => u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]),
    })
}

pub(crate) fn read_u64(
    bytes: &[u8],
    offset: usize,
    endianness: Endianness,
) -> Result<u64, ParserError> {
    let raw = bytes
        .get(offset..offset + 8)
        .ok_or(ParserError::InvalidFormat("expected a 64-bit value"))?;
    Ok(match endianness {
        Endianness::Little => u64::from_le_bytes([
            raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
        ]),
        Endianness::Big => u64::from_be_bytes([
            raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
        ]),
    })
}

pub(crate) fn byte_at(
    bytes: &[u8],
    offset: usize,
    message: &'static str,
) -> Result<u8, ParserError> {
    bytes
        .get(offset)
        .copied()
        .ok_or(ParserError::InvalidFormat(message))
}

pub(crate) fn ensure_len(
    bytes: &[u8],
    min_len: usize,
    message: &'static str,
) -> Result<(), ParserError> {
    if bytes.len() < min_len {
        return Err(ParserError::InvalidFormat(message));
    }
    Ok(())
}

pub(crate) fn decode_text_bytes(
    bytes: &[u8],
    text_encoding_code: u8,
) -> Result<String, ParserError> {
    let text_encoding = TextEncoding::from_code(text_encoding_code).ok_or(
        ParserError::Unsupported(UnsupportedFeature::Encoding(text_encoding_code)),
    )?;
    text_encoding.decode(bytes)
}

fn trim_padded_bytes(bytes: &[u8]) -> &[u8] {
    let end = bytes
        .iter()
        .rposition(|value| *value != 0 && *value != b' ')
        .map_or(0, |index| index + 1);
    &bytes[..end]
}

fn io_error(error: std::io::Error) -> ParserError {
    ParserError::Io(error.to_string())
}
