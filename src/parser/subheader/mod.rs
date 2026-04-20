pub mod pointer;
pub mod signature;
use super::TextRef;

use super::SubheaderRowRef;
use super::constants::{
    SAS_COLUMN_TYPE_CHR, SAS_COLUMN_TYPE_NUM, SAS_COMPRESSION_NONE, SAS_COMPRESSION_ROW,
    SAS_COMPRESSION_ROW_ALT, SAS_COMPRESSION_TRUNC, SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS,
    SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT, SAS_SUBHEADER_SIGNATURE_COLUMN_LIST,
    SAS_SUBHEADER_SIGNATURE_COLUMN_MASK, SAS_SUBHEADER_SIGNATURE_COLUMN_NAME,
    SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE, SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT,
    SAS_SUBHEADER_SIGNATURE_COUNTS, SAS_SUBHEADER_SIGNATURE_ROW_SIZE,
    SAS7BDAT_COMPRESSION_SIGNATURE_RDC, SAS7BDAT_COMPRESSION_SIGNATURE_RLE,
};
use super::contracts::{
    self, ColumnKind, ColumnMetadata, CompressionMode, Endianness, SasColumn, SasMetadata,
    SemanticTypeHint,
};
use super::{
    ParserError, SasLayout, UnsupportedFeature, byte_at, decode_text_bytes, ensure_len, read_u16,
    read_u32,
};

#[derive(Debug, Clone, Default)]
struct ColumnMetadataState {
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

#[derive(Debug, Default)]
pub(crate) struct ParsedMetaPage {
    pub(crate) subheader_rows: Vec<SubheaderRowRef>,
    pub(crate) raw_data_offset: Option<usize>,
}

pub(crate) fn parse_meta_page(
    page: &[u8],
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
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
        if signature == SasSubheaderSignature::ColumnText {
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
                if pointer.is_compressed_data
                    && matches!(signature, SasSubheaderSignature::Unknown(_))
                {
                    if metadata.row_length() != 0 && pointer.len != metadata.row_length() {
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
                    SasSubheaderSignature::RowSize => {
                        parse_row_size_subheader(subheader, metadata, layout)?
                    }
                    SasSubheaderSignature::ColumnSize => {
                        parse_column_size_subheader(subheader, metadata, layout)?
                    }
                    SasSubheaderSignature::Counts
                    | SasSubheaderSignature::ColumnText
                    | SasSubheaderSignature::ColumnList
                    | SasSubheaderSignature::ColumnMask => {}
                    SasSubheaderSignature::ColumnName => {
                        parse_column_name_subheader(subheader, metadata, layout)?
                    }
                    SasSubheaderSignature::ColumnAttrs => {
                        parse_column_attrs_subheader(subheader, metadata, layout)?
                    }
                    SasSubheaderSignature::ColumnFormat => {
                        parse_column_format_subheader(subheader, metadata, layout)?
                    }
                    SasSubheaderSignature::Unknown(other) => {
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
                    compression: effective_subheader_compression(metadata.compression())?,
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
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
) -> Result<(), ParserError> {
    ensure_len(
        subheader,
        layout.row_size_min_len(),
        "row size subheader is truncated",
    )?;
    let offsets = layout.row_size_layout();

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
            SAS7BDAT_COMPRESSION_SIGNATURE_RLE => CompressionMode::Row,
            SAS7BDAT_COMPRESSION_SIGNATURE_RDC => CompressionMode::Binary,
            _ => CompressionMode::Unknown(0xFF),
        };
    }

    Ok(())
}

fn parse_column_size_subheader(
    subheader: &[u8],
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
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
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    metadata
        .text_blobs
        .push(subheader[layout.subheader_data_offset()..].to_vec());
    Ok(())
}

fn parse_column_name_subheader(
    subheader: &[u8],
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    let column_count = match layout.word_size {
        super::WordSize::Bit32 => {
            subheader
                .len()
                .checked_sub(20)
                .ok_or(ParserError::InvalidFormat(
                    "column name subheader is truncated",
                ))?
                / 8
        }
        super::WordSize::Bit64 => {
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
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
) -> Result<(), ParserError> {
    ensure_remainder(subheader, layout)?;
    let column_count = match layout.word_size {
        super::WordSize::Bit32 => {
            subheader
                .len()
                .checked_sub(20)
                .ok_or(ParserError::InvalidFormat(
                    "column attrs subheader is truncated",
                ))?
                / layout.column_attrs_entry_size()
        }
        super::WordSize::Bit64 => {
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
    metadata: &mut SasMetadataAccumulator,
    layout: SasLayout,
) -> Result<(), ParserError> {
    ensure_len(
        subheader,
        layout.column_format_layout().min_len,
        "column format subheader is truncated",
    )?;

    let index = metadata.parsed_format_count;
    ensure_column_capacity(metadata, index + 1);
    let format_layout = layout.column_format_layout();
    metadata.columns[index].format_width = format_layout
        .format_width_offset
        .map(|offset| read_u16(subheader, offset, layout.endianness))
        .transpose()?;
    metadata.columns[index].format_digits = format_layout
        .format_digits_offset
        .map(|offset| read_u16(subheader, offset, layout.endianness))
        .transpose()?;
    metadata.columns[index].format_ref = Some(read_text_ref(
        subheader,
        format_layout.format_ref_offset,
        layout.endianness,
    )?);
    metadata.columns[index].label_ref = Some(read_text_ref(
        subheader,
        format_layout.label_ref_offset,
        layout.endianness,
    )?);
    metadata.parsed_format_count += 1;
    Ok(())
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
    layout: SasLayout,
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

fn finalize_columns(metadata: &SasMetadataAccumulator) -> Result<Vec<SasColumn>, ParserError> {
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

fn parse_subheader_pointers(
    page: &[u8],
    layout: SasLayout,
) -> Result<Vec<SasSubheaderPointer>, ParserError> {
    let page_header_layout = layout.page_header_layout();
    let subheader_count = read_u16(
        page,
        page_header_layout.subheader_count_offset,
        layout.endianness,
    )? as usize;
    let pointer_bytes_len = page_header_layout
        .size
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
    let mut pointer_offset = page_header_layout.size;
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

        pointers.push(SasSubheaderPointer {
            offset,
            len,
            compression,
            is_compressed_data,
        });
        pointer_offset += layout.subheader_pointer_size();
    }

    Ok(pointers)
}

fn subheader_slice(page: &[u8], pointer: SasSubheaderPointer) -> Result<&[u8], ParserError> {
    page.get(pointer.offset..pointer.offset + pointer.len)
        .ok_or(ParserError::InvalidFormat(
            "subheader pointer points outside the page",
        ))
}

fn ensure_column_capacity(metadata: &mut SasMetadataAccumulator, len: usize) {
    if metadata.columns.len() < len {
        metadata.columns.resize(len, ColumnMetadataState::default());
    }
}

fn ensure_remainder(subheader: &[u8], layout: SasLayout) -> Result<(), ParserError> {
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

fn read_subheader_signature(
    subheader: &[u8],
    layout: SasLayout,
) -> Result<SasSubheaderSignature, ParserError> {
    let signature = read_u32(subheader, 0, layout.endianness)?;
    let raw = if layout.word_size == super::WordSize::Bit64
        && layout.endianness == Endianness::Big
        && (signature == 0 || signature == u32::MAX)
    {
        let alternate = read_u32(subheader, 4, layout.endianness)?;
        if alternate != 0 { alternate } else { signature }
    } else {
        signature
    };
    Ok(SasSubheaderSignature::from_raw(raw))
}

fn pointer_compression_mode(compression: u8) -> CompressionMode {
    match compression {
        SAS_COMPRESSION_NONE => CompressionMode::None,
        SAS_COMPRESSION_ROW | SAS_COMPRESSION_ROW_ALT => CompressionMode::Row,
        other => CompressionMode::Unknown(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_metadata() -> SasMetadataAccumulator {
        SasMetadataAccumulator::new(
            String::new(),
            0,
            0,
            super::super::constants::UTF8_ENCODING_CODE,
        )
    }

    #[test]
    fn column_name_subheader_underflow_returns_an_error() {
        let layout = SasLayout::bit32_little();
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
        let layout = SasLayout::bit32_little();
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
        let layout = SasLayout::bit32_little();
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
