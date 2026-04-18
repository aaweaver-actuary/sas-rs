pub(crate) const MAGIC_NUMBER: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xea, 0x81, 0x60,
    0xb3, 0x14, 0x11, 0xcf, 0xbd, 0x92, 0x08, 0x00, 0x09, 0xc7, 0x31, 0x8c, 0x18, 0x1f, 0x10, 0x11,
];

pub(crate) const SAS_ALIGNMENT_OFFSET_0: u8 = 0x22;
pub(crate) const SAS_ALIGNMENT_OFFSET_4: u8 = 0x33;
pub(crate) const SAS_ENDIAN_BIG: u8 = 0x00;
pub(crate) const SAS_ENDIAN_LITTLE: u8 = 0x01;

pub(crate) const SAS_COMPRESSION_NONE: u8 = 0x00;
pub(crate) const SAS_COMPRESSION_TRUNC: u8 = 0x01;
pub(crate) const SAS_COMPRESSION_ROW: u8 = 0x04;
pub(crate) const SAS_COMPRESSION_ROW_ALT: u8 = 0x05;

pub(crate) const UTF8_ENCODING_CODE: u8 = 20;
pub(crate) const LATIN1_ENCODING_CODE: u8 = 29;
pub(crate) const WINDOWS_1252_ENCODING_CODE: u8 = 62;
pub(crate) const DEFAULT_ENCODING_CODE: u8 = 0;

pub(crate) const SAS_PAGE_TYPE_META: u16 = 0x0000;
pub(crate) const SAS_PAGE_TYPE_DATA: u16 = 0x0100;
pub(crate) const SAS_PAGE_TYPE_MIX: u16 = 0x0200;
pub(crate) const SAS_PAGE_TYPE_AMD: u16 = 0x0400;
pub(crate) const SAS_PAGE_TYPE_MASK: u16 = 0x0F00;
pub(crate) const SAS_PAGE_TYPE_COMP: u16 = 0x9000;

pub(crate) const SAS_SUBHEADER_SIGNATURE_ROW_SIZE: u32 = 0xF7F7F7F7;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_SIZE: u32 = 0xF6F6F6F6;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COUNTS: u32 = 0xFFFFFC00;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_FORMAT: u32 = 0xFFFFFBFE;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_MASK: u32 = 0xFFFFFFF8;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_ATTRS: u32 = 0xFFFFFFFC;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_TEXT: u32 = 0xFFFFFFFD;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_LIST: u32 = 0xFFFFFFFE;
pub(crate) const SAS_SUBHEADER_SIGNATURE_COLUMN_NAME: u32 = 0xFFFFFFFF;

pub(crate) const SAS_COLUMN_TYPE_NUM: u8 = 0x01;
pub(crate) const SAS_COLUMN_TYPE_CHR: u8 = 0x02;

pub(crate) const SAS_COMPRESSION_SIGNATURE_RLE: &str = "SASYZCRL";
pub(crate) const SAS_COMPRESSION_SIGNATURE_RDC: &str = "SASYZCR2";

pub struct ParserConstants;
