#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowSizeLayout {
    pub row_length: usize,
    pub row_count: usize,
    pub column_count: usize,
    pub page_row_count: usize,
    pub page_size: usize,
}
