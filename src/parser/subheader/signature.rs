// pub const SAS7BDAT_ROW_SIZE_SUBHEADER_SIGNATURE_CODE: u32 = 0xF7F7F7F7;
// pub const SAS7BDAT_COLUMN_SIZE_SUBHEADER_SIGNATURE_CODE: u32 = 0xF6F6F6F6;
// pub const SAS7BDAT_COUNTS_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFC00;
// pub const SAS7BDAT_COLUMN_FORMAT_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFBFE;
// pub const SAS7BDAT_COLUMN_MASK_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFFF8;
// pub const SAS7BDAT_COLUMN_ATTRS_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFFFC;
// pub const SAS7BDAT_COLUMN_TEXT_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFFFD;
// pub const SAS7BDAT_COLUMN_LIST_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFFFE;
// pub const SAS7BDAT_COLUMN_NAME_SUBHEADER_SIGNATURE_CODE: u32 = 0xFFFFFFFF;

use super::super::constants::{
    SAS7BDAT_COLUMN_ATTRS_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_FORMAT_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_COLUMN_LIST_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_MASK_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_COLUMN_NAME_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COLUMN_SIZE_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_COLUMN_TEXT_SUBHEADER_SIGNATURE_CODE,
    SAS7BDAT_COUNTS_SUBHEADER_SIGNATURE_CODE, SAS7BDAT_ROW_SIZE_SUBHEADER_SIGNATURE_CODE,
};

/// Classified SAS subheader signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasSubheaderSignature {
    /// Row-size metadata subheader.
    RowSize,
    /// Column-size metadata subheader.
    ColumnSize,
    /// Counts metadata subheader.
    Counts,
    /// Column-format metadata subheader.
    ColumnFormat,
    /// Column-mask metadata subheader.
    ColumnMask,
    /// Column-attributes metadata subheader.
    ColumnAttrs,
    /// Column-text metadata subheader.
    ColumnText,
    /// Column-list metadata subheader.
    ColumnList,
    /// Column-name metadata subheader.
    ColumnName,
    /// Unrecognized subheader signature.
    Unknown(u32),
}

impl SasSubheaderSignature {
    /// Classify a raw subheader signature value.
    pub fn from_raw(signature: u32) -> Self {
        match signature {
            SAS7BDAT_ROW_SIZE_SUBHEADER_SIGNATURE_CODE => Self::RowSize,
            SAS7BDAT_COLUMN_SIZE_SUBHEADER_SIGNATURE_CODE => Self::ColumnSize,
            SAS7BDAT_COUNTS_SUBHEADER_SIGNATURE_CODE => Self::Counts,
            SAS7BDAT_COLUMN_FORMAT_SUBHEADER_SIGNATURE_CODE => Self::ColumnFormat,
            SAS7BDAT_COLUMN_ATTRS_SUBHEADER_SIGNATURE_CODE => Self::ColumnAttrs,
            SAS7BDAT_COLUMN_TEXT_SUBHEADER_SIGNATURE_CODE => Self::ColumnText,
            SAS7BDAT_COLUMN_LIST_SUBHEADER_SIGNATURE_CODE => Self::ColumnList,
            SAS7BDAT_COLUMN_NAME_SUBHEADER_SIGNATURE_CODE => Self::ColumnName,
            other
                if (other & SAS7BDAT_COLUMN_MASK_SUBHEADER_SIGNATURE_CODE)
                    == SAS7BDAT_COLUMN_MASK_SUBHEADER_SIGNATURE_CODE =>
            {
                Self::ColumnMask
            }
            other => Self::Unknown(other),
        }
    }
}
