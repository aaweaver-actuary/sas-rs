#![allow(dead_code)]

use sas_rs::parser::contracts::{Endianness, WordSize};
use std::cmp;
use std::fs;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::{SystemTime, UNIX_EPOCH};

const HEADER_SIZE: usize = 8192;
const PAGE_SIZE: usize = 4096;
const PAGE_COUNT_OFFSET: usize = 204;
const HEADER_PREFIX_LEN: usize = PAGE_COUNT_OFFSET + 8 + 4;
const ROW_SIZE_SUBHEADER_LEN: usize = 808;

const MAGIC_NUMBER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea, 0x81, 0x60,
    0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c, 0x18, 0x1f, 0x10, 0x11,
];

const SAS_ALIGNMENT_OFFSET_4: u8 = 0x33;
const SAS_ALIGNMENT_OFFSET_0: u8 = 0x22;
const SAS_ENDIAN_BIG: u8 = 0x00;
const SAS_ENDIAN_LITTLE: u8 = 0x01;

const SAS_PAGE_TYPE_META: u16 = 0x0000;
const SAS_PAGE_TYPE_DATA: u16 = 0x0100;
const SAS_PAGE_TYPE_MIX: u16 = 0x0200;

const SAS_SUBHEADER_SIGNATURE_ROW_SIZE: u32 = 0xF7F7F7F7;
const SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE: u32 = 0xF6F6F6F6;
const SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT: u32 = 0xFFFFFFFD;
const SAS_SUBHEADER_SIGNATURE_COLUMN_NAME: u32 = 0xFFFFFFFF;
const SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS: u32 = 0xFFFFFFFC;
const SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT: u32 = 0xFFFFFBFE;

const SAS_COLUMN_TYPE_NUM: u8 = 0x01;
const SAS_COLUMN_TYPE_CHR: u8 = 0x02;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureLayout {
    pub word_size: WordSize,
    pub endianness: Endianness,
}

impl FixtureLayout {
    pub const fn bit64_little() -> Self {
        Self {
            word_size: WordSize::Bit64,
            endianness: Endianness::Little,
        }
    }

    pub const fn bit64_big() -> Self {
        Self {
            word_size: WordSize::Bit64,
            endianness: Endianness::Big,
        }
    }

    pub const fn bit32_little() -> Self {
        Self {
            word_size: WordSize::Bit32,
            endianness: Endianness::Little,
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

    fn row_size_offsets(self) -> (usize, usize, usize, usize, usize) {
        match self.word_size {
            WordSize::Bit32 => (20, 24, 36, 60, 52),
            WordSize::Bit64 => (40, 48, 72, 120, 104),
        }
    }

    fn numeric_bytes(self, value: f64) -> [u8; 8] {
        match self.endianness {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        }
    }
}

impl Default for FixtureLayout {
    fn default() -> Self {
        Self::bit64_little()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureDefinition {
    pub layout: FixtureLayout,
    pub encoding_code: u8,
    pub header_alignment_padding: bool,
    pub table_name: String,
    pub columns: Vec<FixtureColumn>,
    pub column_metadata: Vec<FixtureColumnMetadata>,
    pub rows: Vec<Vec<FixtureValue>>,
    pub compression_signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FixtureColumnMetadata {
    pub label: Option<String>,
    pub format_name: Option<String>,
    pub informat_name: Option<String>,
    pub format_width: Option<u16>,
    pub format_digits: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureColumn {
    Numeric { name: String, width: usize },
    String { name: String, width: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureValue {
    Numeric(f64),
    NumericBytes(Vec<u8>),
    String(String),
    StringBytes(Vec<u8>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextRef {
    index: u16,
    offset: u16,
    length: u16,
}

#[derive(Debug, Clone, Copy, Default)]
struct FixtureColumnFormatRefs {
    format_ref: Option<TextRef>,
    label_ref: Option<TextRef>,
}

#[derive(Debug, Default)]
pub struct ReadMonitor {
    bytes_read: AtomicUsize,
}

impl ReadMonitor {
    pub fn bytes_read(&self) -> usize {
        self.bytes_read.load(Ordering::SeqCst)
    }

    fn record_read(&self, count: usize) {
        self.bytes_read.fetch_add(count, Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub struct TrackingCursor {
    inner: Cursor<Vec<u8>>,
    monitor: Arc<ReadMonitor>,
}

impl TrackingCursor {
    pub fn new(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> Self {
        Self {
            inner: Cursor::new(bytes),
            monitor,
        }
    }
}

impl Read for TrackingCursor {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let count = self.inner.read(buffer)?;
        self.monitor.record_read(count);
        Ok(count)
    }
}

impl Seek for TrackingCursor {
    fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(position)
    }
}

pub fn supported_fixture_definition() -> FixtureDefinition {
    FixtureDefinition {
        layout: FixtureLayout::default(),
        encoding_code: 20,
        header_alignment_padding: false,
        table_name: "DATASET".to_string(),
        columns: vec![
            FixtureColumn::Numeric {
                name: "customer_id".to_string(),
                width: 8,
            },
            FixtureColumn::String {
                name: "code".to_string(),
                width: 4,
            },
        ],
        column_metadata: vec![
            FixtureColumnMetadata::default(),
            FixtureColumnMetadata::default(),
        ],
        rows: vec![
            vec![
                FixtureValue::Numeric(1.0),
                FixtureValue::String("ABCD".to_string()),
            ],
            vec![
                FixtureValue::Numeric(2.5),
                FixtureValue::String("EFGH".to_string()),
            ],
            vec![
                FixtureValue::Numeric(3.0),
                FixtureValue::String("IJKL".to_string()),
            ],
        ],
        compression_signature: None,
    }
}

pub fn supported_fixture_bytes() -> Vec<u8> {
    build_fixture(&supported_fixture_definition())
}

pub fn bit32_little_endian_fixture_bytes() -> Vec<u8> {
    let mut definition = supported_fixture_definition();
    definition.layout = FixtureLayout::bit32_little();
    build_fixture(&definition)
}

pub fn bit32_little_endian_padded_header_fixture_bytes() -> Vec<u8> {
    let mut definition = supported_fixture_definition();
    definition.layout = FixtureLayout::bit32_little();
    definition.header_alignment_padding = true;
    build_fixture(&definition)
}

pub fn big_endian_fixture_bytes() -> Vec<u8> {
    let mut definition = supported_fixture_definition();
    definition.layout = FixtureLayout::bit64_big();
    build_fixture(&definition)
}

pub fn latin1_fixture_bytes() -> Vec<u8> {
    let mut definition = supported_fixture_definition();
    definition.encoding_code = 29;
    definition.rows = vec![vec![
        FixtureValue::Numeric(1.0),
        FixtureValue::StringBytes(vec![0x43, 0x61, 0x66, 0xE9]),
    ]];
    build_fixture(&definition)
}

pub fn compressed_fixture_bytes() -> Vec<u8> {
    row_compressed_mixed_page_fixture_bytes()
}

pub fn row_compressed_mixed_page_fixture_bytes() -> Vec<u8> {
    let mut definition = supported_fixture_definition();
    definition.rows = vec![
        vec![
            FixtureValue::Numeric(1.0),
            FixtureValue::String("ABCD".to_string()),
        ],
        vec![
            FixtureValue::Numeric(2.5),
            FixtureValue::String("EFGH".to_string()),
        ],
        vec![
            FixtureValue::Numeric(3.0),
            FixtureValue::String("IJKL".to_string()),
        ],
        vec![
            FixtureValue::Numeric(4.25),
            FixtureValue::String("MNOP".to_string()),
        ],
    ];
    build_compressed_fixture(&definition, "SASYZCRL", 1, encode_rle_copy_row)
}

pub fn binary_compressed_fixture_bytes() -> Vec<u8> {
    let definition = supported_fixture_definition();
    build_compressed_fixture(&definition, "SASYZCR2", 0, encode_binary_literal_row)
}

pub fn unsupported_page_type_fixture_bytes(page_type: u16) -> Vec<u8> {
    let mut fixture = supported_fixture_bytes();
    let page_header_size = FixtureLayout::default().page_header_size();
    write_u16(
        &mut fixture[HEADER_SIZE + PAGE_SIZE + page_header_size - 8..],
        0,
        page_type,
        FixtureLayout::default().endianness,
    );
    fixture
}

pub fn malformed_word_size_fixture_bytes(word_size_marker: u8) -> Vec<u8> {
    let mut fixture = bit32_little_endian_fixture_bytes();
    fixture[32] = word_size_marker;
    fixture
}

pub fn write_fixture_file(definition: &FixtureDefinition, path: &Path) {
    fs::write(path, build_fixture(definition)).expect("fixture file should be written");
}

pub fn unique_tmp_path(prefix: &str, extension: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".tmp");
    fs::create_dir_all(&root).expect(".tmp directory should exist");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the unix epoch")
        .as_nanos();
    root.join(format!(
        "{prefix}-{}-{unique}.{extension}",
        std::process::id()
    ))
}

pub fn page_count_for(definition: &FixtureDefinition) -> usize {
    let row_length = definition.columns.iter().map(column_width).sum::<usize>();
    let rows_per_page = (PAGE_SIZE - definition.layout.page_header_size()) / row_length;
    let data_page_count = definition.rows.len().div_ceil(rows_per_page);
    1 + data_page_count
}

pub fn lazy_parse_read_budget(page_count: usize) -> usize {
    HEADER_PREFIX_LEN + page_count * FixtureLayout::default().page_header_size() + PAGE_SIZE
}

pub fn first_batch_read_budget(page_count: usize) -> usize {
    lazy_parse_read_budget(page_count) + PAGE_SIZE
}

pub fn tracked_reader(bytes: Vec<u8>) -> (TrackingCursor, Arc<ReadMonitor>) {
    let monitor = Arc::new(ReadMonitor::default());
    let reader = TrackingCursor::new(bytes, monitor.clone());
    (reader, monitor)
}

pub fn tracked_reader_with_monitor(bytes: Vec<u8>, monitor: Arc<ReadMonitor>) -> TrackingCursor {
    TrackingCursor::new(bytes, monitor)
}

pub fn build_fixture(definition: &FixtureDefinition) -> Vec<u8> {
    assert!(
        !definition.columns.is_empty(),
        "fixture needs at least one column"
    );
    assert!(
        definition
            .rows
            .iter()
            .all(|row| row.len() == definition.columns.len()),
        "each row must match the column count"
    );

    let row_length = definition.columns.iter().map(column_width).sum::<usize>();
    assert!(row_length > 0, "row_length must be greater than zero");

    let rows_per_page = (PAGE_SIZE - definition.layout.page_header_size()) / row_length;
    assert!(rows_per_page > 0, "row_length must fit inside a page");

    let data_page_count = definition.rows.len().div_ceil(rows_per_page);
    let page_count = 1 + data_page_count;

    let mut text_blob = vec![0_u8; 28];
    let mut column_name_refs = Vec::with_capacity(definition.columns.len());
    let column_metadata = normalized_column_metadata(definition);
    let compression_ref = definition
        .compression_signature
        .as_ref()
        .map(|signature| append_text(&mut text_blob, signature));

    for column in &definition.columns {
        column_name_refs.push(append_text(&mut text_blob, column.name()));
    }
    let column_format_refs = column_metadata
        .iter()
        .map(|metadata| FixtureColumnFormatRefs {
            format_ref: metadata
                .format_name
                .as_deref()
                .map(|value| append_text(&mut text_blob, value)),
            label_ref: metadata
                .label
                .as_deref()
                .map(|value| append_text(&mut text_blob, value)),
        })
        .collect::<Vec<_>>();

    let mut output = vec![0_u8; HEADER_SIZE + page_count * PAGE_SIZE];
    write_header(
        &mut output[..HEADER_SIZE],
        definition,
        page_count,
        definition.layout,
    );

    let subheaders = build_subheaders(
        definition,
        &text_blob,
        &column_name_refs,
        &column_metadata,
        &column_format_refs,
        compression_ref,
        row_length,
    );
    let meta_start = HEADER_SIZE;
    write_meta_page(
        &mut output[meta_start..meta_start + PAGE_SIZE],
        &subheaders,
        definition.layout,
    );

    let mut row_cursor = 0;
    for page_index in 0..data_page_count {
        let page_start = HEADER_SIZE + (page_index + 1) * PAGE_SIZE;
        let remaining_rows = definition.rows.len() - row_cursor;
        let page_row_count = cmp::min(rows_per_page, remaining_rows);
        let page_slice = &mut output[page_start..page_start + PAGE_SIZE];
        write_data_page(
            page_slice,
            &definition.rows[row_cursor..row_cursor + page_row_count],
            &definition.columns,
            definition.layout,
        );
        row_cursor += page_row_count;
    }

    output
}

impl FixtureColumn {
    pub fn name(&self) -> &str {
        match self {
            Self::Numeric { name, .. } | Self::String { name, .. } => name,
        }
    }
}

fn build_subheaders(
    definition: &FixtureDefinition,
    text_blob: &[u8],
    column_name_refs: &[TextRef],
    column_metadata: &[FixtureColumnMetadata],
    column_format_refs: &[FixtureColumnFormatRefs],
    compression_ref: Option<TextRef>,
    row_length: usize,
) -> Vec<Vec<u8>> {
    let column_count = definition.columns.len();
    let layout = definition.layout;
    let mut subheaders = vec![
        row_size_subheader(
            row_length,
            definition.rows.len(),
            column_count,
            0,
            compression_ref,
            layout,
        ),
        column_size_subheader(column_count, layout),
        column_text_subheader(text_blob, layout),
        column_name_subheader(column_name_refs, layout),
        column_attrs_subheader(&definition.columns, layout),
    ];

    for (metadata, refs) in column_metadata.iter().zip(column_format_refs.iter()) {
        subheaders.push(column_format_subheader(layout, refs, metadata));
    }

    subheaders
}

fn row_size_subheader(
    row_length: usize,
    row_count: usize,
    column_count: usize,
    page_row_count: usize,
    compression_ref: Option<TextRef>,
    layout: FixtureLayout,
) -> Vec<u8> {
    let mut bytes = vec![0_u8; ROW_SIZE_SUBHEADER_LEN];
    let (
        row_length_offset,
        row_count_offset,
        column_count_offset,
        page_row_count_offset,
        page_size_offset,
    ) = layout.row_size_offsets();

    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_ROW_SIZE,
        layout.endianness,
    );
    write_word(&mut bytes, row_length_offset, row_length as u64, layout);
    write_word(&mut bytes, row_count_offset, row_count as u64, layout);
    write_word(&mut bytes, column_count_offset, column_count as u64, layout);
    write_word(
        &mut bytes,
        page_row_count_offset,
        page_row_count as u64,
        layout,
    );
    write_word(&mut bytes, page_size_offset, PAGE_SIZE as u64, layout);

    if let Some(text_ref) = compression_ref {
        write_text_ref(
            &mut bytes,
            ROW_SIZE_SUBHEADER_LEN - 118,
            text_ref,
            layout.endianness,
        );
    }

    bytes
}

fn column_size_subheader(column_count: usize, layout: FixtureLayout) -> Vec<u8> {
    let mut bytes = vec![0_u8; 24];
    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE,
        layout.endianness,
    );
    write_word(
        &mut bytes,
        layout.subheader_data_offset(),
        column_count as u64,
        layout,
    );
    bytes
}

fn column_text_subheader(text_blob: &[u8], layout: FixtureLayout) -> Vec<u8> {
    let data_offset = layout.subheader_data_offset();
    let mut bytes = vec![0_u8; data_offset + text_blob.len()];
    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT,
        layout.endianness,
    );
    bytes[data_offset..].copy_from_slice(text_blob);
    let remainder = subheader_remainder(bytes.len(), layout);
    write_u16(&mut bytes, data_offset, remainder, layout.endianness);
    let metadata_pad_start = 4 + layout.subheader_data_offset() * 2;
    let metadata_pad_end = metadata_pad_start + 8;
    bytes[metadata_pad_start..metadata_pad_end].fill(b' ');
    bytes
}

fn column_name_subheader(column_name_refs: &[TextRef], layout: FixtureLayout) -> Vec<u8> {
    let data_offset = layout.subheader_data_offset();
    let mut bytes = vec![0_u8; data_offset + 20 + 8 * column_name_refs.len()];
    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_COLUMN_NAME,
        layout.endianness,
    );
    let remainder = subheader_remainder(bytes.len(), layout);
    write_u16(&mut bytes, data_offset, remainder, layout.endianness);

    let mut offset = data_offset + 8;
    for text_ref in column_name_refs {
        write_text_ref(&mut bytes, offset, *text_ref, layout.endianness);
        offset += 8;
    }

    bytes
}

fn column_attrs_subheader(columns: &[FixtureColumn], layout: FixtureLayout) -> Vec<u8> {
    let data_offset = layout.subheader_data_offset();
    let entry_size = layout.column_attrs_entry_size();
    let mut bytes = vec![0_u8; data_offset + 20 + entry_size * columns.len()];
    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS,
        layout.endianness,
    );
    let remainder = subheader_remainder(bytes.len(), layout);
    write_u16(&mut bytes, data_offset, remainder, layout.endianness);

    let mut row_offset = 0_usize;
    let mut entry_offset = data_offset + 8;
    for column in columns {
        write_word(&mut bytes, entry_offset, row_offset as u64, layout);
        write_u32(
            &mut bytes,
            entry_offset + layout.word_size_bytes(),
            column_width(column) as u32,
            layout.endianness,
        );
        write_u16(
            &mut bytes,
            entry_offset + layout.word_size_bytes() + 4,
            if column.name().len() <= 8 { 4 } else { 2048 },
            layout.endianness,
        );
        bytes[entry_offset + layout.word_size_bytes() + 6] = match column {
            FixtureColumn::Numeric { .. } => SAS_COLUMN_TYPE_NUM,
            FixtureColumn::String { .. } => SAS_COLUMN_TYPE_CHR,
        };

        row_offset += column_width(column);
        entry_offset += entry_size;
    }

    bytes
}

fn column_format_subheader(
    layout: FixtureLayout,
    refs: &FixtureColumnFormatRefs,
    metadata: &FixtureColumnMetadata,
) -> Vec<u8> {
    let mut bytes = vec![0_u8; 64];
    write_u32(
        &mut bytes,
        0,
        SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT,
        layout.endianness,
    );
    if layout.word_size == WordSize::Bit64 {
        write_u16(
            &mut bytes,
            24,
            metadata.format_width.unwrap_or(0),
            layout.endianness,
        );
        write_u16(
            &mut bytes,
            26,
            metadata.format_digits.unwrap_or(0),
            layout.endianness,
        );
    }
    if let Some(text_ref) = refs.format_ref {
        write_text_ref(
            &mut bytes,
            match layout.word_size {
                WordSize::Bit64 => 46,
                WordSize::Bit32 => 34,
            },
            text_ref,
            layout.endianness,
        );
    }
    if let Some(text_ref) = refs.label_ref {
        write_text_ref(
            &mut bytes,
            match layout.word_size {
                WordSize::Bit64 => 52,
                WordSize::Bit32 => 40,
            },
            text_ref,
            layout.endianness,
        );
    }
    bytes
}

fn write_header(
    bytes: &mut [u8],
    definition: &FixtureDefinition,
    page_count: usize,
    layout: FixtureLayout,
) {
    bytes[..32].copy_from_slice(&MAGIC_NUMBER);
    bytes[32] = match layout.word_size {
        WordSize::Bit32 => SAS_ALIGNMENT_OFFSET_0,
        WordSize::Bit64 => SAS_ALIGNMENT_OFFSET_4,
    };
    bytes[35] = if definition.header_alignment_padding {
        SAS_ALIGNMENT_OFFSET_4
    } else {
        SAS_ALIGNMENT_OFFSET_0
    };
    bytes[37] = match layout.endianness {
        Endianness::Little => SAS_ENDIAN_LITTLE,
        Endianness::Big => SAS_ENDIAN_BIG,
    };
    bytes[39] = b'1';
    bytes[70] = definition.encoding_code;
    bytes[84..92].copy_from_slice(b"SAS FILE");
    write_padded_ascii(bytes, 92, 32, &definition.table_name);
    bytes[156..164].copy_from_slice(b"DATA    ");
    let header_shift = if definition.header_alignment_padding {
        4
    } else {
        0
    };
    write_u32(
        bytes,
        196 + header_shift,
        HEADER_SIZE as u32,
        layout.endianness,
    );
    write_u32(
        bytes,
        200 + header_shift,
        PAGE_SIZE as u32,
        layout.endianness,
    );
    write_word(bytes, 204 + header_shift, page_count as u64, layout);
    bytes[220..228].copy_from_slice(b"9.0101M0");
    bytes[228..244].copy_from_slice(b"9.0401M6Linux\0\0\0");
}

fn write_meta_page(bytes: &mut [u8], subheaders: &[Vec<u8>], layout: FixtureLayout) {
    bytes.fill(0);
    let page_header_size = layout.page_header_size();
    write_u16(
        bytes,
        page_header_size - 8,
        SAS_PAGE_TYPE_META,
        layout.endianness,
    );
    write_u16(bytes, page_header_size - 6, 0, layout.endianness);
    write_u16(
        bytes,
        page_header_size - 4,
        subheaders.len() as u16,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 2,
        subheaders.len() as u16,
        layout.endianness,
    );

    let mut pointer_offset = page_header_size;
    let mut data_offset = PAGE_SIZE;
    for subheader in subheaders {
        data_offset -= subheader.len();
        write_word(bytes, pointer_offset, data_offset as u64, layout);
        write_word(
            bytes,
            pointer_offset + layout.word_size_bytes(),
            subheader.len() as u64,
            layout,
        );
        bytes[data_offset..data_offset + subheader.len()].copy_from_slice(subheader);
        pointer_offset += layout.subheader_pointer_size();
    }
}

fn build_compressed_fixture(
    definition: &FixtureDefinition,
    compression_signature: &str,
    mix_row_count: usize,
    encoder: fn(&[u8]) -> Vec<u8>,
) -> Vec<u8> {
    assert!(
        mix_row_count <= definition.rows.len(),
        "mix row count must fit within the fixture rows"
    );
    let row_length = definition.columns.iter().map(column_width).sum::<usize>();
    let mut text_blob = vec![0_u8; 28];
    let mut column_name_refs = Vec::with_capacity(definition.columns.len());
    let column_metadata = normalized_column_metadata(definition);
    let compression_ref = Some(append_text(&mut text_blob, compression_signature));

    for column in &definition.columns {
        column_name_refs.push(append_text(&mut text_blob, column.name()));
    }
    let column_format_refs = column_metadata
        .iter()
        .map(|metadata| FixtureColumnFormatRefs {
            format_ref: metadata
                .format_name
                .as_deref()
                .map(|value| append_text(&mut text_blob, value)),
            label_ref: metadata
                .label
                .as_deref()
                .map(|value| append_text(&mut text_blob, value)),
        })
        .collect::<Vec<_>>();

    let compressed_rows = &definition.rows[..definition.rows.len() - mix_row_count];
    let mix_rows = &definition.rows[definition.rows.len() - mix_row_count..];
    let page_count =
        1 + usize::from(!compressed_rows.is_empty()) + usize::from(!mix_rows.is_empty());
    let mut output = vec![0_u8; HEADER_SIZE + page_count * PAGE_SIZE];
    write_header(
        &mut output[..HEADER_SIZE],
        definition,
        page_count,
        definition.layout,
    );

    let subheaders = {
        let column_count = definition.columns.len();
        let layout = definition.layout;
        let mut subheaders = vec![
            row_size_subheader(
                row_length,
                definition.rows.len(),
                column_count,
                mix_rows.len(),
                compression_ref,
                layout,
            ),
            column_size_subheader(column_count, layout),
            column_text_subheader(&text_blob, layout),
            column_name_subheader(&column_name_refs, layout),
            column_attrs_subheader(&definition.columns, layout),
        ];
        for (metadata, refs) in column_metadata.iter().zip(&column_format_refs) {
            subheaders.push(column_format_subheader(layout, refs, metadata));
        }
        subheaders
    };
    write_meta_page(
        &mut output[HEADER_SIZE..HEADER_SIZE + PAGE_SIZE],
        &subheaders,
        definition.layout,
    );

    let mut page_index = 1;
    if !compressed_rows.is_empty() {
        let payloads = compressed_rows
            .iter()
            .map(|row| {
                let mut bytes = vec![0_u8; row_length];
                write_row(&mut bytes, row, &definition.columns, definition.layout);
                encoder(&bytes)
            })
            .collect::<Vec<_>>();
        let page_start = HEADER_SIZE + page_index * PAGE_SIZE;
        write_subheader_row_page(
            &mut output[page_start..page_start + PAGE_SIZE],
            &payloads,
            definition.layout,
        );
        page_index += 1;
    }

    if !mix_rows.is_empty() {
        let page_start = HEADER_SIZE + page_index * PAGE_SIZE;
        write_mix_page(
            &mut output[page_start..page_start + PAGE_SIZE],
            mix_rows,
            &definition.columns,
            definition.layout,
        );
    }

    output
}

fn encode_rle_copy_row(bytes: &[u8]) -> Vec<u8> {
    assert!(
        !bytes.is_empty() && bytes.len() <= 16,
        "test helper only emits short copy rows"
    );
    let mut output = Vec::with_capacity(bytes.len() + 1);
    output.push(0x80 | ((bytes.len() - 1) as u8));
    output.extend_from_slice(bytes);
    output
}

fn encode_binary_literal_row(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len() + bytes.len().div_ceil(16) * 2);
    for chunk in bytes.chunks(16) {
        output.extend_from_slice(&0_u16.to_be_bytes());
        output.extend_from_slice(chunk);
    }
    output
}

fn write_subheader_row_page(bytes: &mut [u8], payloads: &[Vec<u8>], layout: FixtureLayout) {
    bytes.fill(0);
    let page_header_size = layout.page_header_size();
    write_u16(
        bytes,
        page_header_size - 8,
        SAS_PAGE_TYPE_META,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 6,
        payloads.len() as u16,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 4,
        payloads.len() as u16,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 2,
        payloads.len() as u16,
        layout.endianness,
    );

    let mut pointer_offset = page_header_size;
    let mut data_offset = PAGE_SIZE;
    for payload in payloads {
        data_offset -= payload.len();
        write_word(bytes, pointer_offset, data_offset as u64, layout);
        write_word(
            bytes,
            pointer_offset + layout.word_size_bytes(),
            payload.len() as u64,
            layout,
        );
        bytes[pointer_offset + layout.word_size_bytes() * 2] = 0x04;
        bytes[pointer_offset + layout.word_size_bytes() * 2 + 1] = 1;
        bytes[data_offset..data_offset + payload.len()].copy_from_slice(payload);
        pointer_offset += layout.subheader_pointer_size();
    }
}

fn write_mix_page(
    bytes: &mut [u8],
    rows: &[Vec<FixtureValue>],
    columns: &[FixtureColumn],
    layout: FixtureLayout,
) {
    bytes.fill(0);
    let page_header_size = layout.page_header_size();
    write_u16(
        bytes,
        page_header_size - 8,
        SAS_PAGE_TYPE_MIX,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 6,
        rows.len() as u16,
        layout.endianness,
    );
    write_u16(bytes, page_header_size - 4, 0, layout.endianness);
    write_u16(bytes, page_header_size - 2, 0, layout.endianness);

    let row_length = columns.iter().map(column_width).sum::<usize>();
    let mut row_offset = page_header_size;
    for row in rows {
        let row_slice = &mut bytes[row_offset..row_offset + row_length];
        write_row(row_slice, row, columns, layout);
        row_offset += row_length;
    }
}

fn write_data_page(
    bytes: &mut [u8],
    rows: &[Vec<FixtureValue>],
    columns: &[FixtureColumn],
    layout: FixtureLayout,
) {
    bytes.fill(0);
    let page_header_size = layout.page_header_size();
    write_u16(
        bytes,
        page_header_size - 8,
        SAS_PAGE_TYPE_DATA,
        layout.endianness,
    );
    write_u16(
        bytes,
        page_header_size - 6,
        rows.len() as u16,
        layout.endianness,
    );

    let row_length = columns.iter().map(column_width).sum::<usize>();
    let mut row_offset = page_header_size;
    for row in rows {
        let row_slice = &mut bytes[row_offset..row_offset + row_length];
        write_row(row_slice, row, columns, layout);
        row_offset += row_length;
    }
}

fn write_row(
    bytes: &mut [u8],
    row: &[FixtureValue],
    columns: &[FixtureColumn],
    layout: FixtureLayout,
) {
    let mut offset = 0;
    for (value, column) in row.iter().zip(columns.iter()) {
        match (value, column) {
            (FixtureValue::Numeric(number), FixtureColumn::Numeric { width, .. }) => {
                assert_eq!(
                    *width, 8,
                    "FixtureValue::Numeric only supports 8-byte numeric columns"
                );
                bytes[offset..offset + 8].copy_from_slice(&layout.numeric_bytes(*number));
                offset += 8;
            }
            (FixtureValue::NumericBytes(raw_bytes), FixtureColumn::Numeric { width, .. }) => {
                assert_eq!(
                    raw_bytes.len(),
                    *width,
                    "numeric byte payload must match the declared column width"
                );
                bytes[offset..offset + *width].copy_from_slice(raw_bytes);
                offset += *width;
            }
            (FixtureValue::String(value), FixtureColumn::String { width, .. }) => {
                let value_bytes = value.as_bytes();
                assert!(
                    value_bytes.len() <= *width,
                    "string value must fit inside the declared width"
                );

                let target = &mut bytes[offset..offset + *width];
                target.fill(0);
                target[..value_bytes.len()].copy_from_slice(value_bytes);
                offset += *width;
            }
            (FixtureValue::StringBytes(value_bytes), FixtureColumn::String { width, .. }) => {
                assert!(
                    value_bytes.len() <= *width,
                    "string byte payload must fit inside the declared width"
                );

                let target = &mut bytes[offset..offset + *width];
                target.fill(0);
                target[..value_bytes.len()].copy_from_slice(value_bytes);
                offset += *width;
            }
            _ => panic!("row values must match the declared column types"),
        }
    }
}

fn subheader_remainder(len: usize, layout: FixtureLayout) -> u16 {
    (len - (4 + layout.subheader_data_offset() * 2)) as u16
}

fn normalized_column_metadata(definition: &FixtureDefinition) -> Vec<FixtureColumnMetadata> {
    let mut metadata = definition.column_metadata.clone();
    metadata.resize(definition.columns.len(), FixtureColumnMetadata::default());
    metadata.truncate(definition.columns.len());
    metadata
}

pub fn tagged_missing_numeric_bytes(layout: FixtureLayout, tag: char) -> Vec<u8> {
    let mut bytes = f64::NAN.to_ne_bytes();
    bytes[if cfg!(target_endian = "little") { 5 } else { 2 }] = !(tag as u8);
    let value = f64::from_ne_bytes(bytes);
    match layout.endianness {
        Endianness::Little => value.to_le_bytes().to_vec(),
        Endianness::Big => value.to_be_bytes().to_vec(),
    }
}

fn append_text(blob: &mut Vec<u8>, value: &str) -> TextRef {
    let offset = blob.len() as u16;
    let length = value.len() as u16;
    blob.extend_from_slice(value.as_bytes());

    let padding = (4 - (value.len() % 4)) % 4;
    blob.extend(std::iter::repeat_n(b' ', padding));

    TextRef {
        index: 0,
        offset,
        length,
    }
}

fn column_width(column: &FixtureColumn) -> usize {
    match column {
        FixtureColumn::Numeric { width, .. } => *width,
        FixtureColumn::String { width, .. } => *width,
    }
}

fn write_padded_ascii(bytes: &mut [u8], offset: usize, len: usize, value: &str) {
    let target = &mut bytes[offset..offset + len];
    target.fill(b' ');

    let value_bytes = value.as_bytes();
    let copy_len = cmp::min(value_bytes.len(), len);
    target[..copy_len].copy_from_slice(&value_bytes[..copy_len]);
}

fn write_text_ref(bytes: &mut [u8], offset: usize, text_ref: TextRef, endianness: Endianness) {
    write_u16(bytes, offset, text_ref.index, endianness);
    write_u16(bytes, offset + 2, text_ref.offset, endianness);
    write_u16(bytes, offset + 4, text_ref.length, endianness);
}

fn write_word(bytes: &mut [u8], offset: usize, value: u64, layout: FixtureLayout) {
    match layout.word_size {
        WordSize::Bit32 => write_u32(bytes, offset, value as u32, layout.endianness),
        WordSize::Bit64 => write_u64(bytes, offset, value, layout.endianness),
    }
}

fn write_u16(bytes: &mut [u8], offset: usize, value: u16, endianness: Endianness) {
    let encoded = match endianness {
        Endianness::Little => value.to_le_bytes(),
        Endianness::Big => value.to_be_bytes(),
    };
    bytes[offset..offset + 2].copy_from_slice(&encoded);
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32, endianness: Endianness) {
    let encoded = match endianness {
        Endianness::Little => value.to_le_bytes(),
        Endianness::Big => value.to_be_bytes(),
    };
    bytes[offset..offset + 4].copy_from_slice(&encoded);
}

fn write_u64(bytes: &mut [u8], offset: usize, value: u64, endianness: Endianness) {
    let encoded = match endianness {
        Endianness::Little => value.to_le_bytes(),
        Endianness::Big => value.to_be_bytes(),
    };
    bytes[offset..offset + 8].copy_from_slice(&encoded);
}
