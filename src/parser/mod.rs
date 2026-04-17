use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Seek, SeekFrom};

pub mod contracts;

pub use contracts::{
    BoxedParserDataSource, ColumnKind, ColumnMetadata, CompressionMode, Endianness, NumericValue,
    ParsedRow, ParsedSas7bdat, ParsedValue, ParserDataSource, ParserInput, RowBatch,
    SUPPORTED_SUBSET, SasColumn, SasMetadata, SemanticTypeHint, SupportedSubset, WordSize,
};

const MAGIC_NUMBER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea, 0x81, 0x60,
    0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c, 0x18, 0x1f, 0x10, 0x11,
];

const WORD_SIZE_OFFSET: usize = 32;
const ENDIANNESS_OFFSET: usize = 37;
const ENCODING_OFFSET: usize = 70;
const TABLE_NAME_OFFSET: usize = 92;
const TABLE_NAME_LEN: usize = 32;
const HEADER_SIZE_OFFSET: usize = 196;
const PAGE_SIZE_OFFSET: usize = 200;
const PAGE_COUNT_OFFSET: usize = 204;
const HEADER_PREFIX_LEN: usize = PAGE_COUNT_OFFSET + 8;

const SAS_ALIGNMENT_OFFSET_0: u8 = 0x00;
const SAS_ALIGNMENT_OFFSET_4: u8 = 0x33;
const SAS_ENDIAN_BIG: u8 = 0x00;
const SAS_ENDIAN_LITTLE: u8 = 0x01;
const UTF8_ENCODING_CODE: u8 = 20;

const SAS_PAGE_TYPE_META: u16 = 0x0000;
const SAS_PAGE_TYPE_DATA: u16 = 0x0100;
const SAS_PAGE_TYPE_MASK: u16 = 0x0F00;
const SAS_PAGE_TYPE_COMP: u16 = 0x9000;

const SAS_SUBHEADER_SIGNATURE_ROW_SIZE: u32 = 0xF7F7F7F7;
const SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE: u32 = 0xF6F6F6F6;
const SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT: u32 = 0xFFFFFFFD;
const SAS_SUBHEADER_SIGNATURE_COLUMN_NAME: u32 = 0xFFFFFFFF;
const SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS: u32 = 0xFFFFFFFC;
const SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT: u32 = 0xFFFFFBFE;

const SAS_COLUMN_TYPE_NUM: u8 = 0x01;
const SAS_COLUMN_TYPE_CHR: u8 = 0x02;

const SAS_COMPRESSION_SIGNATURE_RLE: &str = "SASYZCRL";
const SAS_COMPRESSION_SIGNATURE_RDC: &str = "SASYZCR2";

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
        parse_supported_subset(input)
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
}

#[derive(Debug, Clone)]
struct PartialMetadata {
    table_name: String,
    file_label: String,
    row_count: usize,
    row_length: usize,
    page_size: usize,
    page_count: usize,
    declared_column_count: Option<usize>,
    parsed_name_count: usize,
    parsed_attr_count: usize,
    text_blobs: Vec<Vec<u8>>,
    columns: Vec<ColumnState>,
    compression: CompressionMode,
}

#[derive(Debug, Clone, Copy)]
struct SubheaderPointer {
    offset: usize,
    len: usize,
    compression: u8,
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
}

impl DecodeLayout {
    fn from_header_prefix(header_prefix: &[u8]) -> Result<Self, ParserError> {
        let word_size = match header_prefix[WORD_SIZE_OFFSET] {
            SAS_ALIGNMENT_OFFSET_0 => WordSize::Bit32,
            SAS_ALIGNMENT_OFFSET_4 => WordSize::Bit64,
            _ => {
                return Err(ParserError::InvalidFormat(
                    "invalid sas7bdat word-size flag",
                ));
            }
        };
        let endianness = match header_prefix[ENDIANNESS_OFFSET] {
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

    fn from_subset(subset: SupportedSubset) -> Self {
        Self {
            word_size: subset.word_size,
            endianness: subset.endianness,
        }
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

    fn row_size_offsets(self) -> RowSizeOffsets {
        match self.word_size {
            WordSize::Bit32 => RowSizeOffsets {
                row_length: 20,
                row_count: 24,
            },
            WordSize::Bit64 => RowSizeOffsets {
                row_length: 40,
                row_count: 48,
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

fn parse_supported_subset(mut input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError> {
    let mut header_prefix = vec![0_u8; HEADER_PREFIX_LEN];
    input
        .reader
        .read_exact(&mut header_prefix)
        .map_err(io_error)?;

    if header_prefix[..MAGIC_NUMBER.len()] != MAGIC_NUMBER {
        return Err(ParserError::InvalidFormat("missing sas7bdat magic number"));
    }

    let layout = DecodeLayout::from_header_prefix(&header_prefix)?;

    if header_prefix[ENCODING_OFFSET] != UTF8_ENCODING_CODE {
        return Err(ParserError::Unsupported(UnsupportedFeature::Encoding(
            header_prefix[ENCODING_OFFSET],
        )));
    }

    let header_size = read_u32(&header_prefix, HEADER_SIZE_OFFSET, layout.endianness)? as usize;
    let page_size = read_u32(&header_prefix, PAGE_SIZE_OFFSET, layout.endianness)? as usize;
    let page_count = layout.read_word(&header_prefix, PAGE_COUNT_OFFSET)? as usize;

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

    let table_name =
        trim_ascii_field(&header_prefix[TABLE_NAME_OFFSET..TABLE_NAME_OFFSET + TABLE_NAME_LEN]);
    let mut metadata = PartialMetadata {
        table_name,
        file_label: String::new(),
        row_count: 0,
        row_length: 0,
        page_size,
        page_count,
        declared_column_count: None,
        parsed_name_count: 0,
        parsed_attr_count: 0,
        text_blobs: Vec::new(),
        columns: Vec::new(),
        compression: CompressionMode::None,
    };
    let mut data_pages = Vec::new();

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
            return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
                CompressionMode::Binary,
            )));
        }

        match page_type & SAS_PAGE_TYPE_MASK {
            SAS_PAGE_TYPE_META => {
                let page = read_page(input.reader.as_mut(), header_size, page_size, page_index)?;
                parse_meta_page(&page, &mut metadata, layout)?;
            }
            SAS_PAGE_TYPE_DATA => data_pages.push(page_index),
            other => {
                return Err(ParserError::Unsupported(UnsupportedFeature::PageType(
                    other,
                )));
            }
        }
    }

    if metadata.compression != CompressionMode::None {
        return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
            metadata.compression,
        )));
    }

    let columns = finalize_columns(&metadata)?;
    let dataset_metadata = SasMetadata {
        subset: contracts::supported_subset(
            layout.word_size,
            layout.endianness,
            CompressionMode::None,
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
        data_pages,
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
            && self.next_data_page < self.data_pages.len()
            && self.decoded_row_count() < self.metadata.row_count
        {
            self.load_next_data_page()?;
        }

        if self.pending_rows.is_empty()
            && self.next_data_page == self.data_pages.len()
            && self.decoded_row_count() != self.metadata.row_count
        {
            return Err(ParserError::InvalidFormat(
                "data pages did not yield the expected row count",
            ));
        }

        Ok(())
    }

    fn load_next_data_page(&mut self) -> Result<(), ParserError> {
        let layout = DecodeLayout::from_subset(self.metadata.subset);
        let page_index =
            *self
                .data_pages
                .get(self.next_data_page)
                .ok_or(ParserError::InvalidFormat(
                    "data page index is out of bounds",
                ))?;
        self.next_data_page += 1;

        let page = read_page(
            self.reader.as_mut(),
            self.header_size,
            self.metadata.page_size,
            page_index,
        )?;
        let page_row_count =
            read_u16(&page, layout.page_header_size() - 6, layout.endianness)? as usize;
        let data = &page[layout.page_header_size()..];

        for row_index in 0..page_row_count {
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
                .ok_or(ParserError::InvalidFormat("data page row is truncated"))?;
            self.pending_rows.push_back(parse_row(row, &self.metadata)?);
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
) -> Result<(), ParserError> {
    let pointers = parse_subheader_pointers(page, layout)?;

    for pointer in &pointers {
        let subheader = subheader_slice(page, *pointer)?;
        let signature = read_u32(subheader, 0, layout.endianness)?;
        if signature == SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT {
            parse_column_text_subheader(subheader, metadata, layout)?;
        }
    }

    for pointer in &pointers {
        if pointer.compression != 0 {
            return Err(ParserError::Unsupported(UnsupportedFeature::Compression(
                CompressionMode::Unknown(pointer.compression),
            )));
        }

        let subheader = subheader_slice(page, *pointer)?;
        let signature = read_u32(subheader, 0, layout.endianness)?;
        match signature {
            SAS_SUBHEADER_SIGNATURE_ROW_SIZE => {
                parse_row_size_subheader(subheader, metadata, layout)?
            }
            SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE => {
                parse_column_size_subheader(subheader, metadata, layout)?
            }
            SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT => {}
            SAS_SUBHEADER_SIGNATURE_COLUMN_NAME => {
                parse_column_name_subheader(subheader, metadata, layout)?
            }
            SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS => {
                parse_column_attrs_subheader(subheader, metadata, layout)?
            }
            SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT => parse_column_format_subheader(subheader)?,
            other => {
                return Err(ParserError::Unsupported(
                    UnsupportedFeature::SubheaderSignature(other),
                ));
            }
        }
    }

    Ok(())
}

fn parse_row_size_subheader(
    subheader: &[u8],
    metadata: &mut PartialMetadata,
    layout: DecodeLayout,
) -> Result<(), ParserError> {
    ensure_len(subheader, 128, "row size subheader is truncated")?;
    let offsets = layout.row_size_offsets();

    metadata.row_length = layout.read_word(subheader, offsets.row_length)? as usize;
    metadata.row_count = layout.read_word(subheader, offsets.row_count)? as usize;

    let file_label_ref = read_text_ref(subheader, subheader.len() - 130, layout.endianness)?;
    if file_label_ref.length > 0 {
        metadata.file_label = resolve_text(&metadata.text_blobs, file_label_ref)?;
    }

    let compression_ref = read_text_ref(subheader, subheader.len() - 118, layout.endianness)?;
    if compression_ref.length > 0 {
        let compression = resolve_text(&metadata.text_blobs, compression_ref)?;
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
    ensure_len(subheader, 16, "column size subheader is truncated")?;
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
    let column_count = (subheader.len() - (layout.subheader_data_offset() + 20)) / 8;
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
    let column_count = (subheader.len() - (layout.subheader_data_offset() + 20))
        / layout.column_attrs_entry_size();
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

fn parse_column_format_subheader(subheader: &[u8]) -> Result<(), ParserError> {
    ensure_len(subheader, 58, "column format subheader is truncated")?;
    Ok(())
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

        columns.push(SasColumn {
            name: resolve_text(&metadata.text_blobs, name_ref)?,
            kind,
            offset,
            width,
            semantic_type: SemanticTypeHint::Deferred,
            metadata: ColumnMetadata::default(),
        });
    }

    Ok(columns)
}

fn parse_row(row: &[u8], metadata: &SasMetadata) -> Result<ParsedRow, ParserError> {
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
            ColumnKind::String => ParsedValue::String(trim_ascii_field(raw_value)),
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
        8 => Ok(ParsedValue::Numeric(
            match endianness {
                Endianness::Little => f64::from_le_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
                Endianness::Big => f64::from_be_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
            }
            .into(),
        )),
        width => Err(ParserError::Unsupported(UnsupportedFeature::NumericWidth(
            width as u32,
        ))),
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
    let expected_remainder = subheader.len().saturating_sub(20) as u16;
    let remainder = read_u16(subheader, layout.subheader_data_offset(), layout.endianness)?;
    if remainder != expected_remainder {
        return Err(ParserError::InvalidFormat(
            "subheader remainder does not match the supported layout",
        ));
    }

    Ok(())
}

fn resolve_text(text_blobs: &[Vec<u8>], text_ref: TextRef) -> Result<String, ParserError> {
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
    std::str::from_utf8(slice)
        .map(|value| value.to_string())
        .map_err(|_| ParserError::InvalidFormat("expected utf-8 text in the supported subset"))
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

fn trim_ascii_field(bytes: &[u8]) -> String {
    let end = bytes
        .iter()
        .rposition(|value| *value != 0 && *value != b' ')
        .map_or(0, |index| index + 1);
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

fn io_error(error: std::io::Error) -> ParserError {
    ParserError::Io(error.to_string())
}
