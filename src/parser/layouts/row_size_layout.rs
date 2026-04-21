/// Offsets used when decoding a row-size subheader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowSizeLayout {
    /// Offset of the declared row length.
    pub row_length: usize,
    /// Offset of the declared row count.
    pub row_count: usize,
    /// Offset of the declared column count.
    pub column_count: usize,
    /// Offset of the declared per-page row count.
    pub page_row_count: usize,
    /// Offset of the declared page size.
    pub page_size: usize,
}
