#![allow(dead_code)]

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
const PAGE_HEADER_SIZE: usize = 40;
const PAGE_COUNT_OFFSET: usize = 204;
const HEADER_PREFIX_LEN: usize = PAGE_COUNT_OFFSET + 8;
const SUBHEADER_POINTER_SIZE: usize = 24;

const MAGIC_NUMBER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea, 0x81, 0x60,
    0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c, 0x18, 0x1f, 0x10, 0x11,
];

const SAS_PAGE_TYPE_META: u16 = 0x0000;
const SAS_PAGE_TYPE_DATA: u16 = 0x0100;

const SAS_SUBHEADER_SIGNATURE_ROW_SIZE: u32 = 0xF7F7F7F7;
const SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE: u32 = 0xF6F6F6F6;
const SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT: u32 = 0xFFFFFFFD;
const SAS_SUBHEADER_SIGNATURE_COLUMN_NAME: u32 = 0xFFFFFFFF;
const SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS: u32 = 0xFFFFFFFC;
const SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT: u32 = 0xFFFFFBFE;

const SAS_COLUMN_TYPE_NUM: u8 = 0x01;
const SAS_COLUMN_TYPE_CHR: u8 = 0x02;

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureDefinition {
    pub table_name: String,
    pub columns: Vec<FixtureColumn>,
    pub rows: Vec<Vec<FixtureValue>>,
    pub compression_signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureColumn {
    Numeric { name: String },
    String { name: String, width: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureValue {
    Numeric(f64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextRef {
    index: u16,
    offset: u16,
    length: u16,
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
        table_name: "DATASET".to_string(),
        columns: vec![
            FixtureColumn::Numeric {
                name: "customer_id".to_string(),
            },
            FixtureColumn::String {
                name: "code".to_string(),
                width: 4,
            },
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

pub fn big_endian_fixture_bytes() -> Vec<u8> {
    let mut bytes = supported_fixture_bytes();
    bytes[37] = 0x00;
    bytes
}

pub fn compressed_fixture_bytes() -> Vec<u8> {
    let mut fixture = supported_fixture_definition();
    fixture.compression_signature = Some("SASYZCRL".to_string());
    build_fixture(&fixture)
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
    let rows_per_page = (PAGE_SIZE - PAGE_HEADER_SIZE) / row_length;
    let data_page_count = definition.rows.len().div_ceil(rows_per_page);
    1 + data_page_count
}

pub fn lazy_parse_read_budget(page_count: usize) -> usize {
    HEADER_PREFIX_LEN + page_count * PAGE_HEADER_SIZE + PAGE_SIZE
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

    let rows_per_page = (PAGE_SIZE - PAGE_HEADER_SIZE) / row_length;
    assert!(rows_per_page > 0, "row_length must fit inside a page");

    let data_page_count = definition.rows.len().div_ceil(rows_per_page);
    let page_count = 1 + data_page_count;

    let mut text_blob = vec![0_u8; 28];
    let mut column_name_refs = Vec::with_capacity(definition.columns.len());

    let compression_ref = definition
        .compression_signature
        .as_ref()
        .map(|signature| append_text(&mut text_blob, signature));

    for column in &definition.columns {
        column_name_refs.push(append_text(&mut text_blob, column.name()));
    }

    let mut output = vec![0_u8; HEADER_SIZE + page_count * PAGE_SIZE];
    write_header(&mut output[..HEADER_SIZE], definition, page_count);

    let subheaders = build_subheaders(
        definition,
        &text_blob,
        &column_name_refs,
        compression_ref,
        row_length,
    );
    let meta_start = HEADER_SIZE;
    write_meta_page(&mut output[meta_start..meta_start + PAGE_SIZE], &subheaders);

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
        );
        row_cursor += page_row_count;
    }

    output
}

impl FixtureColumn {
    pub fn name(&self) -> &str {
        match self {
            Self::Numeric { name } | Self::String { name, .. } => name,
        }
    }
}

fn build_subheaders(
    definition: &FixtureDefinition,
    text_blob: &[u8],
    column_name_refs: &[TextRef],
    compression_ref: Option<TextRef>,
    row_length: usize,
) -> Vec<Vec<u8>> {
    let column_count = definition.columns.len();
    let mut subheaders = vec![
        row_size_subheader(
            row_length,
            definition.rows.len(),
            column_count,
            compression_ref,
        ),
        column_size_subheader(column_count),
        column_text_subheader(text_blob),
        column_name_subheader(column_name_refs),
        column_attrs_subheader(&definition.columns),
    ];

    for _ in &definition.columns {
        subheaders.push(column_format_subheader());
    }

    subheaders
}

fn row_size_subheader(
    row_length: usize,
    row_count: usize,
    column_count: usize,
    compression_ref: Option<TextRef>,
) -> Vec<u8> {
    let mut bytes = vec![0_u8; 808];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_ROW_SIZE);
    write_u64(&mut bytes, 40, row_length as u64);
    write_u64(&mut bytes, 48, row_count as u64);
    write_u64(&mut bytes, 72, column_count as u64);
    write_u64(&mut bytes, 104, PAGE_SIZE as u64);

    if let Some(text_ref) = compression_ref {
        write_text_ref(&mut bytes, 808 - 118, text_ref);
    }

    bytes
}

fn column_size_subheader(column_count: usize) -> Vec<u8> {
    let mut bytes = vec![0_u8; 24];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE);
    write_u64(&mut bytes, 8, column_count as u64);
    bytes
}

fn column_text_subheader(text_blob: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 8 + text_blob.len()];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT);
    bytes[8..].copy_from_slice(text_blob);
    let remainder = (bytes.len() - 20) as u16;
    write_u16(&mut bytes, 8, remainder);
    bytes[20..28].fill(b' ');
    bytes
}

fn column_name_subheader(column_name_refs: &[TextRef]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 28 + 8 * column_name_refs.len()];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_COLUMN_NAME);
    let remainder = (bytes.len() - 20) as u16;
    write_u16(&mut bytes, 8, remainder);

    let mut offset = 16;
    for text_ref in column_name_refs {
        write_text_ref(&mut bytes, offset, *text_ref);
        offset += 8;
    }

    bytes
}

fn column_attrs_subheader(columns: &[FixtureColumn]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 28 + 16 * columns.len()];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS);
    let remainder = (bytes.len() - 20) as u16;
    write_u16(&mut bytes, 8, remainder);

    let mut row_offset = 0_usize;
    let mut entry_offset = 16;
    for column in columns {
        write_u64(&mut bytes, entry_offset, row_offset as u64);
        write_u32(&mut bytes, entry_offset + 8, column_width(column) as u32);
        write_u16(
            &mut bytes,
            entry_offset + 12,
            if column.name().len() <= 8 { 4 } else { 2048 },
        );
        bytes[entry_offset + 14] = match column {
            FixtureColumn::Numeric { .. } => SAS_COLUMN_TYPE_NUM,
            FixtureColumn::String { .. } => SAS_COLUMN_TYPE_CHR,
        };

        row_offset += column_width(column);
        entry_offset += 16;
    }

    bytes
}

fn column_format_subheader() -> Vec<u8> {
    let mut bytes = vec![0_u8; 64];
    write_u32(&mut bytes, 0, SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT);
    bytes
}

fn write_header(bytes: &mut [u8], definition: &FixtureDefinition, page_count: usize) {
    bytes[..32].copy_from_slice(&MAGIC_NUMBER);
    bytes[32] = 0x33;
    bytes[35] = 0x22;
    bytes[37] = 0x01;
    bytes[39] = b'1';
    bytes[70] = 20;
    bytes[84..92].copy_from_slice(b"SAS FILE");
    write_padded_ascii(bytes, 92, 32, &definition.table_name);
    bytes[156..164].copy_from_slice(b"DATA    ");
    write_u32(bytes, 196, HEADER_SIZE as u32);
    write_u32(bytes, 200, PAGE_SIZE as u32);
    write_u64(bytes, 204, page_count as u64);
    bytes[220..228].copy_from_slice(b"9.0101M0");
    bytes[228..244].copy_from_slice(b"9.0401M6Linux\0\0\0");
}

fn write_meta_page(bytes: &mut [u8], subheaders: &[Vec<u8>]) {
    bytes.fill(0);
    write_u16(bytes, PAGE_HEADER_SIZE - 8, SAS_PAGE_TYPE_META);
    write_u16(bytes, PAGE_HEADER_SIZE - 6, 0);
    write_u16(bytes, PAGE_HEADER_SIZE - 4, subheaders.len() as u16);
    write_u16(bytes, PAGE_HEADER_SIZE - 2, subheaders.len() as u16);

    let mut pointer_offset = PAGE_HEADER_SIZE;
    let mut data_offset = PAGE_SIZE;
    for subheader in subheaders {
        data_offset -= subheader.len();
        write_u64(bytes, pointer_offset, data_offset as u64);
        write_u64(bytes, pointer_offset + 8, subheader.len() as u64);
        bytes[data_offset..data_offset + subheader.len()].copy_from_slice(subheader);
        pointer_offset += SUBHEADER_POINTER_SIZE;
    }
}

fn write_data_page(bytes: &mut [u8], rows: &[Vec<FixtureValue>], columns: &[FixtureColumn]) {
    bytes.fill(0);
    write_u16(bytes, PAGE_HEADER_SIZE - 8, SAS_PAGE_TYPE_DATA);
    write_u16(bytes, PAGE_HEADER_SIZE - 6, rows.len() as u16);

    let row_length = columns.iter().map(column_width).sum::<usize>();
    let mut row_offset = PAGE_HEADER_SIZE;
    for row in rows {
        let row_slice = &mut bytes[row_offset..row_offset + row_length];
        write_row(row_slice, row, columns);
        row_offset += row_length;
    }
}

fn write_row(bytes: &mut [u8], row: &[FixtureValue], columns: &[FixtureColumn]) {
    let mut offset = 0;
    for (value, column) in row.iter().zip(columns.iter()) {
        match (value, column) {
            (FixtureValue::Numeric(number), FixtureColumn::Numeric { .. }) => {
                bytes[offset..offset + 8].copy_from_slice(&number.to_le_bytes());
                offset += 8;
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
            _ => panic!("row values must match the declared column types"),
        }
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
        FixtureColumn::Numeric { .. } => 8,
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

fn write_text_ref(bytes: &mut [u8], offset: usize, text_ref: TextRef) {
    write_u16(bytes, offset, text_ref.index);
    write_u16(bytes, offset + 2, text_ref.offset);
    write_u16(bytes, offset + 4, text_ref.length);
}

fn write_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
