use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Seek, SeekFrom};

use encoding_rs::{Encoding, GB18030, ISO_8859_15, WINDOWS_1251, WINDOWS_1252};

pub mod constants;
pub mod contracts;
pub mod offsets;

pub use contracts::{
    BoxedParserDataSource, ColumnKind, ColumnMetadata, CompressionMode, Endianness, NumericValue,
    ParsedRow, ParsedSas7bdat, ParsedValue, ParserDataSource, ParserInput, RowBatch,
    SUPPORTED_SUBSET, SasColumn, SasMetadata, SasMissingTag, SemanticTypeHint, SupportedSubset,
    WordSize,
};

use self::contracts::{PageRowSource, SubheaderRowRef};

pub use constants::ParserConstants;
pub use offsets::ParserOffsets;

use self::constants::{
    DEFAULT_ENCODING_CODE, LATIN1_ENCODING_CODE, MAGIC_NUMBER, SAS_ALIGNMENT_OFFSET_0,
    SAS_ALIGNMENT_OFFSET_4, SAS_COLUMN_TYPE_CHR, SAS_COLUMN_TYPE_NUM, SAS_COMPRESSION_NONE,
    SAS_COMPRESSION_ROW, SAS_COMPRESSION_ROW_ALT, SAS_COMPRESSION_SIGNATURE_RDC,
    SAS_COMPRESSION_SIGNATURE_RLE,
    SAS_COMPRESSION_TRUNC, SAS_ENDIAN_BIG, SAS_ENDIAN_LITTLE, SAS_PAGE_TYPE_AMD,
    SAS_PAGE_TYPE_COMP, SAS_PAGE_TYPE_DATA, SAS_PAGE_TYPE_MASK, SAS_PAGE_TYPE_META,
    SAS_PAGE_TYPE_MIX, SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS, SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT,
    SAS_SUBHEADER_SIGNATURE_COLUMN_LIST, SAS_SUBHEADER_SIGNATURE_COLUMN_MASK,
    SAS_SUBHEADER_SIGNATURE_COLUMN_NAME, SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE,
    SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT, SAS_SUBHEADER_SIGNATURE_COUNTS,
    SAS_SUBHEADER_SIGNATURE_ROW_SIZE, UTF8_ENCODING_CODE, WINDOWS_1252_ENCODING_CODE,
};

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

#[derive(Debug, Clone, Copy, Default)]
struct TextRef {
    index: u16,
    offset: usize,
    length: usize,
}

#[derive(Debug, Clone, Default)]
struct ColumnState {
    name_ref: Option<TextRef>,
    kind: Option<ColumnKind>,
    offset: Option<usize>,
    width: Option<usize>,
    label_ref: Option<TextRef>,
    format_ref: Option<TextRef>,
    format_width: Option<u16>,
    format_digits: Option<u16>,
    informat_name: Option<String>,
}

#[derive(Debug, Clone)]
struct PartialMetadata {
    table_name: String,
    file_label: String,
    row_count: usize,
    row_length: usize,
    page_row_count: usize,
    page_size: usize,
    page_count: usize,
    text_encoding_code: u8,
    declared_column_count: Option<usize>,
    parsed_name_count: usize,
    parsed_attr_count: usize,
    parsed_format_count: usize,
    text_blobs: Vec<Vec<u8>>,
    columns: Vec<ColumnState>,
    compression: CompressionMode,
}

#[derive(Debug, Clone, Copy)]
struct SubheaderPointer {
    offset: usize,
    len: usize,
    compression: u8,
    is_compressed_data: bool,
}

#[derive(Debug, Clone, Copy)]
struct DecodeLayout {
    word_size: WordSize,
    endianness: Endianness,
}

#[derive(Debug, Clone, Copy)]
struct RowSizeOffsets {
    row_length: usize,
    row_count: usize,
    page_row_count: usize,
}

#[derive(Debug, Default)]
struct ParsedMetaPage {
    subheader_rows: Vec<SubheaderRowRef>,
    raw_data_offset: Option<usize>,
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

impl DecodeLayout {
    fn from_header_prefix(
        header_prefix: &[u8],
        offsets: &ParserOffsets,
    ) -> Result<Self, ParserError> {
        let word_size = match header_prefix[offsets.word_size] {
            SAS_ALIGNMENT_OFFSET_0 => WordSize::Bit32,
            SAS_ALIGNMENT_OFFSET_4 => WordSize::Bit64,
            _ => {
                return Err(ParserError::InvalidFormat(
                    "invalid sas7bdat word-size flag",
                ));
            }
        };
        let endianness = match header_prefix[offsets.endianness] {
            SAS_ENDIAN_LITTLE => Endianness::Little,
            SAS_ENDIAN_BIG => Endianness::Big,
            _ => {
                return Err(ParserError::InvalidFormat(
                    "invalid sas7bdat endianness flag",
                ));
            }
        };

        Ok(Self {
            word_size,
            endianness,
        })
    }

    fn word_size_bytes(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 4,
            WordSize::Bit64 => 8,
        }
    }

    fn page_header_size(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 24,
            WordSize::Bit64 => 40,
        }
    }

    fn subheader_pointer_size(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 12,
            WordSize::Bit64 => 24,
        }
    }

    fn subheader_data_offset(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 4,
            WordSize::Bit64 => 8,
        }
    }

    fn column_attrs_entry_size(self) -> usize {
        self.word_size_bytes() + 8
    }

    fn subheader_signature_size(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 4,
            WordSize::Bit64 => 8,
        }
    }

    fn row_size_min_len(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 190,
            WordSize::Bit64 => 250,
        }
    }

    fn column_format_min_len(self) -> usize {
        match self.word_size {
            WordSize::Bit32 => 46,
            WordSize::Bit64 => 58,
        }
    }

    fn row_size_offsets(self) -> RowSizeOffsets {
        match self.word_size {
            WordSize::Bit32 => RowSizeOffsets {
                row_length: 20,
                row_count: 24,
                page_row_count: 60,
            },
            WordSize::Bit64 => RowSizeOffsets {
                row_length: 40,
                row_count: 48,
                page_row_count: 120,
            },
        }
    }

    fn read_word(self, bytes: &[u8], offset: usize) -> Result<u64, ParserError> {
        match self.word_size {
            WordSize::Bit32 => Ok(read_u32(bytes, offset, self.endianness)? as u64),
            WordSize::Bit64 => read_u64(bytes, offset, self.endianness),
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

    if header_prefix[..MAGIC_NUMBER.len()] != MAGIC_NUMBER {
        return Err(ParserError::InvalidFormat("missing sas7bdat magic number"));
    }

    let layout = DecodeLayout::from_header_prefix(&header_prefix, offsets)?;
    let text_encoding_code =
        *header_prefix
            .get(offsets.encoding)
            .ok_or(ParserError::InvalidFormat(
                "sas7bdat header prefix is truncated",
            ))?;
    let _text_encoding = TextEncoding::from_code(text_encoding_code).ok_or(
        ParserError::Unsupported(UnsupportedFeature::Encoding(text_encoding_code)),
    )?;

    let header_size = read_u32(
        &header_prefix,
        offsets.header_size_offset(&header_prefix),
        layout.endianness,
    )? as usize;
    let page_size = read_u32(
        &header_prefix,
        offsets.page_size_offset(&header_prefix),
        layout.endianness,
    )? as usize;
    let page_count =
        layout.read_word(&header_prefix, offsets.page_count_offset(&header_prefix))? as usize;

    if header_size < 1024 || page_size < 1024 {
        return Err(ParserError::InvalidFormat(
            "header_size and page_size must both be at least 1024 bytes",
        ));
    }

    let expected_len = header_size
        .checked_add(page_size.saturating_mul(page_count))
        .ok_or(ParserError::InvalidFormat("sas7bdat file size overflowed"))?;
    let file_len = input.reader.seek(SeekFrom::End(0)).map_err(io_error)? as usize;
    if file_len < expected_len {
        return Err(ParserError::InvalidFormat("sas7bdat file is truncated"));
    }

    let table_name = decode_text_bytes(
        &header_prefix[offsets.table_name_start..offsets.table_name_end],
        text_encoding_code,
    )?;

    let mut metadata = PartialMetadata {
        table_name,
        file_label: String::new(),
        row_count: 0,
        row_length: 0,
        page_row_count: 0,
        page_size,
        page_count,
        text_encoding_code,
        declared_column_count: None,
        parsed_name_count: 0,
        parsed_attr_count: 0,
        parsed_format_count: 0,
        text_blobs: Vec::new(),
        columns: Vec::new(),
        compression: CompressionMode::None,
    };
    let mut row_sources = Vec::new();

    for page_index in 0..page_count {
        let page_header = read_page_header(
            input.reader.as_mut(),
            header_size,
            page_size,
            page_index,
            layout,
        )?;
        let page_type = read_u16(
            &page_header,
            layout.page_header_size() - 8,
            layout.endianness,
        )?;
        if (page_type & SAS_PAGE_TYPE_COMP) != 0 {
            let raw_row_count = read_u16(
                &page_header,
                layout.page_header_size() - 6,
                layout.endianness,
            )? as usize;
            row_sources.push(PageRowSource {
                page_index,
                raw_data_offset: Some(layout.page_header_size()),
                raw_row_count,
                subheader_rows: Vec::new(),
            });
            continue;
        }

        match page_type & SAS_PAGE_TYPE_MASK {
            SAS_PAGE_TYPE_META | SAS_PAGE_TYPE_MIX | SAS_PAGE_TYPE_AMD => {
                let page = read_page(input.reader.as_mut(), header_size, page_size, page_index)?;
                let parsed_page = parse_meta_page(
                    &page,
                    &mut metadata,
                    layout,
                    (page_type & SAS_PAGE_TYPE_MASK) == SAS_PAGE_TYPE_MIX,
                )?;
                let raw_row_count = if (page_type & SAS_PAGE_TYPE_MASK) == SAS_PAGE_TYPE_MIX {
                    metadata.page_row_count
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
            SAS_PAGE_TYPE_DATA => {
                let raw_row_count = read_u16(
                    &page_header,
                    layout.page_header_size() - 6,
                    layout.endianness,
                )? as usize;
                row_sources.push(PageRowSource {
                    page_index,
                    raw_data_offset: Some(layout.page_header_size()),
                    raw_row_count,
                    subheader_rows: Vec::new(),
                });
            }
            other => {
                return Err(ParserError::Unsupported(UnsupportedFeature::PageType(
                    other,
                )));
            }
        }
    }

    if matches!(metadata.compression, CompressionMode::Unknown(_)) {
        return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
            metadata.compression,
        )));
    }

    let columns = finalize_columns(&metadata)?;
    let dataset_metadata = SasMetadata {
        subset: contracts::supported_subset(
            layout.word_size,
            layout.endianness,
            metadata.compression,
        ),
        table_name: metadata.table_name.clone(),
        file_label: metadata.file_label.clone(),
        row_count: metadata.row_count,
        row_length: metadata.row_length,
        page_size: metadata.page_size,
        page_count: metadata.page_count,
        columns,
    };

    Ok(ParsedSas7bdat::new_streaming(
        dataset_metadata,
        input.reader,
        header_size,
        row_sources,
        text_encoding_code,
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
            let row = parse_subheader_row(&page, subheader_row, self.metadata.row_length)?;
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

fn parse_meta_page(
    page: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
    allow_mix_raw: bool,
) -> Result<ParsedMetaPage, ParserError> {
    let pointers = parse_subheader_pointers(page, layout)?;
    let mut parsed_page = ParsedMetaPage::default();

    for pointer in &pointers {
        if pointer.len == 0 || pointer.compression == SAS_COMPRESSION_TRUNC {
            continue;
        }
        if pointer.compression != SAS_COMPRESSION_NONE {
            continue;
        }

        let subheader = subheader_slice(page, *pointer)?;
        let signature = read_subheader_signature(subheader, layout)?;
        if signature == SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT {
            parse_column_text_subheader(subheader, metadata, layout)?;
        }
    }

    for pointer in &pointers {
        if pointer.len == 0 || pointer.compression == SAS_COMPRESSION_TRUNC {
            continue;
        }

        match pointer.compression {
            SAS_COMPRESSION_NONE => {
                let subheader = subheader_slice(page, *pointer)?;
                let signature = read_subheader_signature(subheader, layout)?;
                if pointer.is_compressed_data && !signature_is_recognized(signature) {
                    if metadata.row_length != 0 && pointer.len != metadata.row_length {
                        return Err(ParserError::InvalidFormat(
                            "row subheader length does not match the declared row length",
                        ));
                    }
                    parsed_page.subheader_rows.push(SubheaderRowRef {
                        offset: pointer.offset,
                        len: pointer.len,
                        compression: CompressionMode::None,
                    });
                    continue;
                }

                match signature {
                    SAS_SUBHEADER_SIGNATURE_ROW_SIZE => {
                        parse_row_size_subheader(subheader, metadata, layout)?
                    }
                    SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE => {
                        parse_column_size_subheader(subheader, metadata, layout)?
                    }
                    SAS_SUBHEADER_SIGNATURE_COUNTS
                    | SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT
                    | SAS_SUBHEADER_SIGNATURE_COLUMN_LIST => {}
                    SAS_SUBHEADER_SIGNATURE_COLUMN_NAME => {
                        parse_column_name_subheader(subheader, metadata, layout)?
                    }
                    SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS => {
                        parse_column_attrs_subheader(subheader, metadata, layout)?
                    }
                    SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT => {
                        parse_column_format_subheader(subheader, metadata, layout)?
                    }
                    other
                        if (other & SAS_SUBHEADER_SIGNATURE_COLUMN_MASK)
                            == SAS_SUBHEADER_SIGNATURE_COLUMN_MASK => {}
                    other => {
                        return Err(ParserError::Unsupported(
                            UnsupportedFeature::SubheaderSignature(other),
                        ));
                    }
                }
            }
            SAS_COMPRESSION_ROW | SAS_COMPRESSION_ROW_ALT => {
                parsed_page.subheader_rows.push(SubheaderRowRef {
                    offset: pointer.offset,
                    len: pointer.len,
                    compression: effective_subheader_compression(metadata.compression)?,
                });
            }
            other => {
                return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
                    pointer_compression_mode(other),
                )));
            }
        }
    }

    if allow_mix_raw {
        parsed_page.raw_data_offset = Some(mix_raw_data_offset(page, layout, pointers.len())?);
    }

    Ok(parsed_page)
}

fn parse_row_size_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_len(
        subheader,
        layout.row_size_min_len(),
        "row size subheader is truncated",
    )?;
    let offsets = layout.row_size_offsets();

    metadata.row_length = layout.read_word(subheader, offsets.row_length)? as usize;
    metadata.row_count = layout.read_word(subheader, offsets.row_count)? as usize;
    metadata.page_row_count = layout.read_word(subheader, offsets.page_row_count)? as usize;

    let file_label_offset = subheader
        .len()
        .checked_sub(130)
        .ok_or(ParserError::InvalidFormat(
            "row size subheader is truncated",
        ))?;
    let file_label_ref = read_text_ref(subheader, file_label_offset, layout.endianness)?;
    if file_label_ref.length > 0 {
        metadata.file_label = resolve_text(
            &metadata.text_blobs,
            file_label_ref,
            metadata.text_encoding_code,
        )?;
    }

    let compression_offset = subheader
        .len()
        .checked_sub(118)
        .ok_or(ParserError::InvalidFormat(
            "row size subheader is truncated",
        ))?;
    let compression_ref = read_text_ref(subheader, compression_offset, layout.endianness)?;
    if compression_ref.length > 0 {
        let compression = resolve_text(
            &metadata.text_blobs,
            compression_ref,
            metadata.text_encoding_code,
        )?;
        metadata.compression = match compression.as_str() {
            SAS_COMPRESSION_SIGNATURE_RLE => CompressionMode::Row,
            SAS_COMPRESSION_SIGNATURE_RDC => CompressionMode::Binary,
            _ => CompressionMode::Unknown(0xFF),
        };
    }

    Ok(())
}

fn parse_column_size_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_len(
        subheader,
        layout.subheader_data_offset() + layout.word_size_bytes(),
        "column size subheader is truncated",
    )?;
    let column_count = layout.read_word(subheader, layout.subheader_data_offset())? as usize;
    metadata.declared_column_count = Some(column_count);
    ensure_column_capacity(metadata, column_count);
    Ok(())
}

fn parse_column_text_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    metadata
        .text_blobs
        .push(subheader[layout.subheader_data_offset()..].to_vec());
    Ok(())
}

fn parse_column_name_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    let column_count = match layout.word_size {
        WordSize::Bit32 => {
            subheader
                .len()
                .checked_sub(20)
                .ok_or(ParserError::InvalidFormat(
                    "column name subheader is truncated",
                ))?
                / 8
        }
        WordSize::Bit64 => {
            subheader
                .len()
                .checked_sub(28)
                .ok_or(ParserError::InvalidFormat(
                    "column name subheader is truncated",
                ))?
                / 8
        }
    };
    let end = metadata.parsed_name_count + column_count;
    ensure_column_capacity(metadata, end);

    let mut offset = layout.subheader_data_offset() + 8;
    for index in metadata.parsed_name_count..end {
        metadata.columns[index].name_ref =
            Some(read_text_ref(subheader, offset, layout.endianness)?);
        offset += 8;
    }
    metadata.parsed_name_count = end;

    Ok(())
}

fn parse_column_attrs_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    let column_count = match layout.word_size {
        WordSize::Bit32 => {
            subheader
                .len()
                .checked_sub(20)
                .ok_or(ParserError::InvalidFormat(
                    "column attrs subheader is truncated",
                ))?
                / layout.column_attrs_entry_size()
        }
        WordSize::Bit64 => {
            subheader
                .len()
                .checked_sub(28)
                .ok_or(ParserError::InvalidFormat(
                    "column attrs subheader is truncated",
                ))?
                / layout.column_attrs_entry_size()
        }
    };
    let end = metadata.parsed_attr_count + column_count;
    ensure_column_capacity(metadata, end);

    let mut offset = layout.subheader_data_offset() + 8;
    for index in metadata.parsed_attr_count..end {
        metadata.columns[index].offset = Some(layout.read_word(subheader, offset)? as usize);
        let width = read_u32(
            subheader,
            offset + layout.word_size_bytes(),
            layout.endianness,
        )? as usize;
        let kind = match byte_at(
            subheader,
            offset + layout.word_size_bytes() + 6,
            "column attrs subheader is truncated",
        )? {
            SAS_COLUMN_TYPE_NUM => ColumnKind::Numeric,
            SAS_COLUMN_TYPE_CHR => ColumnKind::String,
            code => {
                return Err(ParserError::Unsupported(UnsupportedFeature::ColumnType(
                    code,
                )));
            }
        };
        metadata.columns[index].width = Some(width);
        metadata.columns[index].kind = Some(kind);
        offset += layout.column_attrs_entry_size();
    }
    metadata.parsed_attr_count = end;

    Ok(())
}

fn parse_column_format_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_len(
        subheader,
        layout.column_format_min_len(),
        "column format subheader is truncated",
    )?;

    let index = metadata.parsed_format_count;
    ensure_column_capacity(metadata, index + 1);
    metadata.columns[index].format_width = match layout.word_size {
        WordSize::Bit64 => Some(read_u16(subheader, 24, layout.endianness)?),
        WordSize::Bit32 => None,
    };
    metadata.columns[index].format_digits = match layout.word_size {
        WordSize::Bit64 => Some(read_u16(subheader, 26, layout.endianness)?),
        WordSize::Bit32 => None,
    };
    metadata.columns[index].format_ref = Some(read_text_ref(
        subheader,
        match layout.word_size {
            WordSize::Bit64 => 46,
            WordSize::Bit32 => 34,
        },
        layout.endianness,
    )?);
    metadata.columns[index].label_ref = Some(read_text_ref(
        subheader,
        match layout.word_size {
            WordSize::Bit64 => 52,
            WordSize::Bit32 => 40,
        },
        layout.endianness,
    )?);
    metadata.parsed_format_count += 1;
    Ok(())
}

fn signature_is_recognized(signature: u32) -> bool {
    matches!(
        signature,
        SAS_SUBHEADER_SIGNATURE_ROW_SIZE
            | SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE
            | SAS_SUBHEADER_SIGNATURE_COUNTS
            | SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT
            | SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS
            | SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT
            | SAS_SUBHEADER_SIGNATURE_COLUMN_LIST
            | SAS_SUBHEADER_SIGNATURE_COLUMN_NAME
    ) || (signature & SAS_SUBHEADER_SIGNATURE_COLUMN_MASK) == SAS_SUBHEADER_SIGNATURE_COLUMN_MASK
}

fn effective_subheader_compression(
    metadata_compression: CompressionMode,
) -> Result<CompressionMode, ParserError> {
    match metadata_compression {
        CompressionMode::Row | CompressionMode::Binary => Ok(metadata_compression),
        other => Err(ParserError::Unsupported(UnsupportedFeature::Compression(
            other,
        ))),
    }
}

fn mix_raw_data_offset(
    page: &[u8],
    layout: DecodeLayout,
    pointer_count: usize,
) -> Result<usize, ParserError> {
    let mut data_offset = layout
        .page_header_size()
        .checked_add(pointer_count.saturating_mul(layout.subheader_pointer_size()))
        .ok_or(ParserError::InvalidFormat(
            "mix page pointer table overflowed",
        ))?;
    if data_offset > page.len() {
        return Err(ParserError::InvalidFormat(
            "mix page pointer table exceeds the page size",
        ));
    }
    if data_offset % 8 == 4 && data_offset + 4 <= page.len() {
        let padding = &page[data_offset..data_offset + 4];
        if padding == [0, 0, 0, 0] || padding == [b' ', b' ', b' ', b' '] {
            data_offset += 4;
        }
    }
    Ok(data_offset)
}

fn parse_subheader_row(
    page: &[u8],
    subheader_row: &SubheaderRowRef,
    row_length: usize,
) -> Result<Vec<u8>, ParserError> {
    let end = subheader_row
        .offset
        .checked_add(subheader_row.len)
        .ok_or(ParserError::InvalidFormat("subheader row range overflowed"))?;
    let payload = page
        .get(subheader_row.offset..end)
        .ok_or(ParserError::InvalidFormat(
            "subheader row range is outside the page",
        ))?;
    match subheader_row.compression {
        CompressionMode::None => {
            if payload.len() != row_length {
                return Err(ParserError::InvalidFormat(
                    "row subheader length does not match the declared row length",
                ));
            }
            Ok(payload.to_vec())
        }
        CompressionMode::Row => decompress_row_rle(payload, row_length),
        CompressionMode::Binary => decompress_row_binary(payload, row_length),
        CompressionMode::Unknown(code) => Err(ParserError::Unsupported(
            UnsupportedFeature::Compression(CompressionMode::Unknown(code)),
        )),
    }
}

fn decompress_row_rle(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {
    const COMMAND_LENGTHS: [usize; 16] = [1, 1, 0, 0, 2, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0];
    let mut input_offset = 0;
    let mut output = Vec::with_capacity(row_length);

    while input_offset < payload.len() {
        let control = payload[input_offset];
        input_offset += 1;
        let command = (control & 0xF0) >> 4;
        let length = (control & 0x0F) as usize;
        let command_len = COMMAND_LENGTHS[command as usize];
        if input_offset + command_len > payload.len() {
            return Err(ParserError::InvalidFormat(
                "row-compressed payload is truncated",
            ));
        }

        let mut copy_len = 0_usize;
        let mut insert_len = 0_usize;
        let mut insert_byte = 0_u8;

        match command {
            0 => {
                copy_len = payload[input_offset] as usize + 64 + length * 256;
                input_offset += 1;
            }
            1 => {
                copy_len = payload[input_offset] as usize + 64 + length * 256 + 4096;
                input_offset += 1;
            }
            2 => copy_len = length + 96,
            4 => {
                insert_len = payload[input_offset] as usize + 18 + length * 256;
                insert_byte = payload[input_offset + 1];
                input_offset += 2;
            }
            5 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = b'@';
                input_offset += 1;
            }
            6 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = b' ';
                input_offset += 1;
            }
            7 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = 0;
                input_offset += 1;
            }
            8 => copy_len = length + 1,
            9 => copy_len = length + 17,
            10 => copy_len = length + 33,
            11 => copy_len = length + 49,
            12 => {
                insert_len = length + 3;
                insert_byte = payload[input_offset];
                input_offset += 1;
            }
            13 => {
                insert_len = length + 2;
                insert_byte = b'@';
            }
            14 => {
                insert_len = length + 2;
                insert_byte = b' ';
            }
            15 => {
                insert_len = length + 2;
                insert_byte = 0;
            }
            _ => unreachable!(),
        }

        if copy_len != 0 {
            if output.len() + copy_len > row_length {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload exceeds the declared row length",
                ));
            }
            if input_offset + copy_len > payload.len() {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload is truncated",
                ));
            }
            output.extend_from_slice(&payload[input_offset..input_offset + copy_len]);
            input_offset += copy_len;
        }

        if insert_len != 0 {
            if output.len() + insert_len > row_length {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload exceeds the declared row length",
                ));
            }
            output.extend(std::iter::repeat_n(insert_byte, insert_len));
        }
    }

    if output.len() != row_length {
        return Err(ParserError::InvalidFormat(
            "row-compressed payload did not decompress to the declared row length",
        ));
    }

    Ok(output)
}

fn decompress_row_binary(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {
    let mut input_offset = 0;
    let mut output = Vec::with_capacity(row_length);

    while input_offset + 2 <= payload.len() {
        let prefix = u16::from_be_bytes([payload[input_offset], payload[input_offset + 1]]);
        input_offset += 2;

        for bit_index in 0..16 {
            if output.len() == row_length {
                break;
            }

            let is_control = (prefix & (1 << (15 - bit_index))) != 0;
            if !is_control {
                if input_offset >= payload.len() {
                    break;
                }
                if output.len() + 1 > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                output.push(payload[input_offset]);
                input_offset += 1;
                continue;
            }

            if input_offset + 2 > payload.len() {
                return Err(ParserError::InvalidFormat(
                    "binary-compressed payload is truncated",
                ));
            }

            let marker = payload[input_offset];
            let next = payload[input_offset + 1];
            input_offset += 2;
            let mut insert_len = 0_usize;
            let mut copy_len = 0_usize;
            let mut insert_byte = 0_u8;
            let mut back_offset = 0_usize;

            if marker <= 0x0F {
                insert_len = 3 + marker as usize;
                insert_byte = next;
            } else if (marker >> 4) == 1 {
                if input_offset >= payload.len() {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload is truncated",
                    ));
                }
                insert_len = 19 + (marker & 0x0F) as usize + next as usize * 16;
                insert_byte = payload[input_offset];
                input_offset += 1;
            } else if (marker >> 4) == 2 {
                if input_offset >= payload.len() {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload is truncated",
                    ));
                }
                copy_len = 16 + payload[input_offset] as usize;
                input_offset += 1;
                back_offset = 3 + (marker & 0x0F) as usize + next as usize * 16;
            } else {
                copy_len = (marker >> 4) as usize;
                back_offset = 3 + (marker & 0x0F) as usize + next as usize * 16;
            }

            if insert_len != 0 {
                if output.len() + insert_len > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                output.extend(std::iter::repeat_n(insert_byte, insert_len));
            } else if copy_len != 0 {
                if output.len() < back_offset || copy_len > back_offset {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload contains an invalid back-reference",
                    ));
                }
                if output.len() + copy_len > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                let start = output.len() - back_offset;
                for index in 0..copy_len {
                    let byte = output[start + index];
                    output.push(byte);
                }
            }
        }
    }

    if output.len() != row_length {
        return Err(ParserError::InvalidFormat(
            "binary-compressed payload did not decompress to the declared row length",
        ));
    }

    Ok(output)
}

fn finalize_columns(metadata: &PartialMetadata) -> Result<Vec<SasColumn>, ParserError> {
    let declared_column_count =
        metadata
            .declared_column_count
            .ok_or(ParserError::InvalidFormat(
                "column size subheader is missing",
            ))?;
    if metadata.row_length == 0 {
        return Err(ParserError::InvalidFormat("row size subheader is missing"));
    }

    let mut columns = Vec::with_capacity(declared_column_count);
    for index in 0..declared_column_count {
        let column = metadata
            .columns
            .get(index)
            .ok_or(ParserError::InvalidFormat("column metadata is incomplete"))?;
        let name_ref = column.name_ref.ok_or(ParserError::InvalidFormat(
            "column name metadata is incomplete",
        ))?;
        let width = column.width.ok_or(ParserError::InvalidFormat(
            "column width metadata is incomplete",
        ))?;
        let kind = column.kind.clone().ok_or(ParserError::InvalidFormat(
            "column type metadata is incomplete",
        ))?;
        let offset = column.offset.ok_or(ParserError::InvalidFormat(
            "column offset metadata is incomplete",
        ))?;

        match kind {
            ColumnKind::Numeric if width == 0 => {
                return Err(ParserError::InvalidFormat(
                    "numeric column width must be greater than zero",
                ));
            }
            ColumnKind::Numeric if width > 8 => {
                return Err(ParserError::Unsupported(UnsupportedFeature::NumericWidth(
                    width as u32,
                )));
            }
            ColumnKind::String if width == 0 => {
                return Err(ParserError::InvalidFormat(
                    "string column width must be greater than zero",
                ));
            }
            _ => {}
        }

        let format_name = column
            .format_ref
            .filter(|text_ref| text_ref.length > 0)
            .map(|text_ref| {
                resolve_text(&metadata.text_blobs, text_ref, metadata.text_encoding_code)
            })
            .transpose()?
            .map(|name| decorate_format_name(name, column.format_width, column.format_digits));
        let label = column
            .label_ref
            .filter(|text_ref| text_ref.length > 0)
            .map(|text_ref| {
                resolve_text(&metadata.text_blobs, text_ref, metadata.text_encoding_code)
            })
            .transpose()?;
        let column_metadata = ColumnMetadata {
            label,
            format_name: format_name.clone(),
            informat_name: column.informat_name.clone(),
        };

        columns.push(SasColumn {
            name: resolve_text(&metadata.text_blobs, name_ref, metadata.text_encoding_code)?,
            kind,
            offset,
            width,
            semantic_type: semantic_type_from_metadata(&column_metadata),
            metadata: column_metadata,
        });
    }

    Ok(columns)
}

fn parse_row(
    row: &[u8],
    metadata: &SasMetadata,
    text_encoding_code: u8,
) -> Result<ParsedRow, ParserError> {
    let mut values = Vec::with_capacity(metadata.columns.len());

    for column in &metadata.columns {
        let end = column
            .offset
            .checked_add(column.width)
            .ok_or(ParserError::InvalidFormat("column range overflowed"))?;
        let raw_value = row
            .get(column.offset..end)
            .ok_or(ParserError::InvalidFormat("column value is truncated"))?;

        let value = match column.kind {
            ColumnKind::Numeric => parse_numeric_value(raw_value, metadata.subset.endianness)?,
            ColumnKind::String => {
                ParsedValue::String(decode_text_bytes(raw_value, text_encoding_code)?)
            }
        };
        values.push(value);
    }

    Ok(ParsedRow { values })
}

fn parse_numeric_value(
    raw_value: &[u8],
    endianness: Endianness,
) -> Result<ParsedValue, ParserError> {
    match raw_value.len() {
        0 => Err(ParserError::InvalidFormat(
            "numeric value width must be greater than zero",
        )),
        1..=7 => Ok(ParsedValue::Numeric(NumericValue::deferred_bytes(
            raw_value.to_vec(),
        ))),
        8 => {
            let raw_bits = match endianness {
                Endianness::Little => u64::from_le_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
                Endianness::Big => u64::from_be_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
            };
            let value = f64::from_bits(raw_bits);
            Ok(ParsedValue::Numeric(NumericValue::Float64 {
                value,
                raw_bits,
                missing_tag: decode_sas_missing_tag(value, raw_bits),
            }))
        }
        width => Err(ParserError::Unsupported(UnsupportedFeature::NumericWidth(
            width as u32,
        ))),
    }
}

fn decorate_format_name(
    mut format_name: String,
    format_width: Option<u16>,
    format_digits: Option<u16>,
) -> String {
    if let Some(width) = format_width.filter(|width| *width != 0) {
        format_name.push_str(&width.to_string());
    }
    if let Some(digits) = format_digits.filter(|digits| *digits != 0) {
        format_name.push('.');
        format_name.push_str(&digits.to_string());
    }
    format_name
}

fn semantic_type_from_metadata(metadata: &ColumnMetadata) -> SemanticTypeHint {
    metadata
        .format_name
        .as_deref()
        .and_then(semantic_type_from_format_name)
        .or_else(|| {
            metadata
                .informat_name
                .as_deref()
                .and_then(semantic_type_from_format_name)
        })
        .unwrap_or(SemanticTypeHint::Deferred)
}

fn semantic_type_from_format_name(format_name: &str) -> Option<SemanticTypeHint> {
    let upper = format_name.trim().to_ascii_uppercase();
    const DATETIME_PREFIXES: &[&str] = &[
        "DATETIME", "DT", "DATEAMPM", "MDYAMPM", "NLDATMT", "NLDATM", "IS8601DN", "IS8601DT",
        "IS8601DZ", "B8601DN", "B8601DT", "B8601DX", "B8601DZ", "B8601LX", "E8601DN", "E8601DT",
        "E8601DX", "E8601DZ", "E8601LX",
    ];
    const DATE_PREFIXES: &[&str] = &[
        "DATE", "NLDATE", "DOWNAME", "IS8601DA", "B8601DA", "E8601DA", "DAY", "WEEK", "MON", "QTR",
        "YEAR", "MMDDYY", "DDMMYY", "YYMMDD", "MMYY", "YYMM", "YY", "WORDDAT", "NENGO", "JULIAN",
        "JULDAY", "PDJULG", "PDJULI",
    ];
    const TIME_PREFIXES: &[&str] = &[
        "TIME", "TIMEAMPM", "NLTIM", "TOD", "IS8601T", "B8601T", "B8601LZ", "E8601T", "E8601LZ",
    ];
    const DURATION_PREFIXES: &[&str] = &["HOUR", "HHMM", "MMSS", "DURATION"];

    if DATETIME_PREFIXES
        .iter()
        .any(|prefix| upper.starts_with(prefix))
    {
        Some(SemanticTypeHint::DateTime)
    } else if DATE_PREFIXES.iter().any(|prefix| upper.starts_with(prefix)) {
        Some(SemanticTypeHint::Date)
    } else if TIME_PREFIXES.iter().any(|prefix| upper.starts_with(prefix)) {
        Some(SemanticTypeHint::Time)
    } else if DURATION_PREFIXES
        .iter()
        .any(|prefix| upper.starts_with(prefix))
    {
        Some(SemanticTypeHint::Duration)
    } else {
        None
    }
}

fn decode_sas_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {
    if !value.is_nan() {
        return None;
    }

    let tag = !((raw_bits >> 40) & 0xFF) as u8;
    match tag {
        0 => Some(SasMissingTag::Underscore),
        2..=27 => Some(SasMissingTag::Letter((b'A' + (tag - 2)) as char)),
        b'_' => Some(SasMissingTag::Underscore),
        b'A'..=b'Z' => Some(SasMissingTag::Letter(tag as char)),
        _ => Some(SasMissingTag::Dot),
    }
}

fn parse_subheader_pointers(
    page: &[u8],
    layout: DecodeLayout,
) -> Result<Vec<SubheaderPointer>, ParserError> {
    let subheader_count =
        read_u16(page, layout.page_header_size() - 4, layout.endianness)? as usize;
    let pointer_bytes_len = layout
        .page_header_size()
        .checked_add(subheader_count.saturating_mul(layout.subheader_pointer_size()))
        .ok_or(ParserError::InvalidFormat(
            "subheader pointer table overflowed",
        ))?;
    if pointer_bytes_len > page.len() {
        return Err(ParserError::InvalidFormat(
            "subheader pointer table exceeds the page size",
        ));
    }

    let mut pointers = Vec::with_capacity(subheader_count);
    let mut pointer_offset = layout.page_header_size();
    for _ in 0..subheader_count {
        let offset = layout.read_word(page, pointer_offset)? as usize;
        let len = layout.read_word(page, pointer_offset + layout.word_size_bytes())? as usize;
        let compression = byte_at(
            page,
            pointer_offset + layout.word_size_bytes() * 2,
            "subheader pointer is truncated",
        )?;
        let is_compressed_data = byte_at(
            page,
            pointer_offset + layout.word_size_bytes() * 2 + 1,
            "subheader pointer is truncated",
        )? != 0;

        if offset < pointer_bytes_len
            || offset.checked_add(len).is_none()
            || offset + len > page.len()
        {
            return Err(ParserError::InvalidFormat(
                "subheader pointer points outside the page",
            ));
        }

        pointers.push(SubheaderPointer {
            offset,
            len,
            compression,
            is_compressed_data,
        });
        pointer_offset += layout.subheader_pointer_size();
    }

    Ok(pointers)
}

fn subheader_slice(page: &[u8], pointer: SubheaderPointer) -> Result<&[u8], ParserError> {
    page.get(pointer.offset..pointer.offset + pointer.len)
        .ok_or(ParserError::InvalidFormat(
            "subheader pointer points outside the page",
        ))
}

fn ensure_column_capacity(metadata: &mut PartialMetadata, len: usize) {
    if metadata.columns.len() < len {
        metadata.columns.resize(len, ColumnState::default());
    }
}

fn ensure_remainder(subheader: &[u8], layout: DecodeLayout) -> Result<(), ParserError> {
    let expected_remainder = subheader
        .len()
        .checked_sub(4 + layout.subheader_signature_size() * 2)
        .ok_or(ParserError::InvalidFormat(
            "subheader remainder does not match the supported layout",
        ))? as u16;
    let remainder = read_u16(subheader, layout.subheader_data_offset(), layout.endianness)?;
    if remainder != expected_remainder {
        return Err(ParserError::InvalidFormat(
            "subheader remainder does not match the supported layout",
        ));
    }

    Ok(())
}

fn resolve_text(
    text_blobs: &[Vec<u8>],
    text_ref: TextRef,
    text_encoding_code: u8,
) -> Result<String, ParserError> {
    if text_ref.length == 0 {
        return Ok(String::new());
    }

    let blob = text_blobs
        .get(text_ref.index as usize)
        .ok_or(ParserError::InvalidFormat(
            "text reference index is out of bounds",
        ))?;
    let end = text_ref
        .offset
        .checked_add(text_ref.length)
        .ok_or(ParserError::InvalidFormat(
            "text reference range overflowed",
        ))?;
    let slice = blob
        .get(text_ref.offset..end)
        .ok_or(ParserError::InvalidFormat(
            "text reference range is out of bounds",
        ))?;
    decode_text_bytes(slice, text_encoding_code)
}

fn read_page_header(
    reader: &mut dyn ParserDataSource,
    header_size: usize,
    page_size: usize,
    page_index: usize,
    layout: DecodeLayout,
) -> Result<Vec<u8>, ParserError> {
    let offset = page_offset(header_size, page_size, page_index)?;
    let mut header = vec![0_u8; layout.page_header_size()];
    read_exact_at(reader, offset, &mut header)?;
    Ok(header)
}

fn read_page(
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

fn read_text_ref(
    bytes: &[u8],
    offset: usize,
    endianness: Endianness,
) -> Result<TextRef, ParserError> {
    Ok(TextRef {
        index: read_u16(bytes, offset, endianness)?,
        offset: read_u16(bytes, offset + 2, endianness)? as usize,
        length: read_u16(bytes, offset + 4, endianness)? as usize,
    })
}

fn read_u16(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u16, ParserError> {
    let raw = bytes
        .get(offset..offset + 2)
        .ok_or(ParserError::InvalidFormat("expected a 16-bit value"))?;
    Ok(match endianness {
        Endianness::Little => u16::from_le_bytes([raw[0], raw[1]]),
        Endianness::Big => u16::from_be_bytes([raw[0], raw[1]]),
    })
}

fn read_u32(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u32, ParserError> {
    let raw = bytes
        .get(offset..offset + 4)
        .ok_or(ParserError::InvalidFormat("expected a 32-bit value"))?;
    Ok(match endianness {
        Endianness::Little => u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]),
        Endianness::Big => u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]),
    })
}

fn read_u64(bytes: &[u8], offset: usize, endianness: Endianness) -> Result<u64, ParserError> {
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

fn byte_at(bytes: &[u8], offset: usize, message: &'static str) -> Result<u8, ParserError> {
    bytes
        .get(offset)
        .copied()
        .ok_or(ParserError::InvalidFormat(message))
}

fn ensure_len(bytes: &[u8], min_len: usize, message: &'static str) -> Result<(), ParserError> {
    if bytes.len() < min_len {
        return Err(ParserError::InvalidFormat(message));
    }
    Ok(())
}

fn pointer_compression_mode(compression: u8) -> CompressionMode {
    match compression {
        SAS_COMPRESSION_NONE => CompressionMode::None,
        SAS_COMPRESSION_ROW | SAS_COMPRESSION_ROW_ALT => CompressionMode::Row,
        other => CompressionMode::Unknown(other),
    }
}

fn read_subheader_signature(subheader: &[u8], layout: DecodeLayout) -> Result<u32, ParserError> {
    let signature = read_u32(subheader, 0, layout.endianness)?;
    if layout.word_size == WordSize::Bit64
        && layout.endianness == Endianness::Big
        && (signature == 0 || signature == u32::MAX)
    {
        let alternate = read_u32(subheader, 4, layout.endianness)?;
        if alternate != 0 {
            return Ok(alternate);
        }
    }
    Ok(signature)
}

fn decode_text_bytes(bytes: &[u8], text_encoding_code: u8) -> Result<String, ParserError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_metadata() -> PartialMetadata {
        PartialMetadata {
            table_name: String::new(),
            file_label: String::new(),
            row_count: 0,
            row_length: 0,
            page_row_count: 0,
            page_size: 0,
            page_count: 0,
            text_encoding_code: UTF8_ENCODING_CODE,
            declared_column_count: None,
            parsed_name_count: 0,
            parsed_attr_count: 0,
            parsed_format_count: 0,
            text_blobs: Vec::new(),
            columns: Vec::new(),
            compression: CompressionMode::None,
        }
    }

    #[test]
    fn column_name_subheader_underflow_returns_an_error() {
        let layout = DecodeLayout {
            word_size: WordSize::Bit32,
            endianness: Endianness::Little,
        };
        let mut metadata = empty_metadata();
        let subheader = vec![0_u8; layout.subheader_data_offset() + 8];

        let error = parse_column_name_subheader(&subheader, &mut metadata, layout)
            .expect_err("short column-name payloads should be rejected without panicking");

        assert_eq!(
            error,
            ParserError::InvalidFormat("column name subheader is truncated")
        );
    }

    #[test]
    fn column_attrs_subheader_underflow_returns_an_error() {
        let layout = DecodeLayout {
            word_size: WordSize::Bit32,
            endianness: Endianness::Little,
        };
        let mut metadata = empty_metadata();
        let subheader = vec![0_u8; layout.subheader_data_offset() + 8];

        let error = parse_column_attrs_subheader(&subheader, &mut metadata, layout)
            .expect_err("short column-attrs payloads should be rejected without panicking");

        assert_eq!(
            error,
            ParserError::InvalidFormat("column attrs subheader is truncated")
        );
    }

    #[test]
    fn row_size_subheader_short_tail_returns_an_error() {
        let layout = DecodeLayout {
            word_size: WordSize::Bit32,
            endianness: Endianness::Little,
        };
        let mut metadata = empty_metadata();
        let subheader = vec![0_u8; layout.row_size_min_len() - 1];

        let error = parse_row_size_subheader(&subheader, &mut metadata, layout)
            .expect_err("short row-size payloads should be rejected without panicking");

        assert_eq!(
            error,
            ParserError::InvalidFormat("row size subheader is truncated")
        );
    }
}
