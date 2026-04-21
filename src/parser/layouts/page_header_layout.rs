/// Offsets used when decoding a page header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageHeaderLayout {
    /// Total byte width of the page header.
    pub size: usize,
    /// Offset of the raw page-type field.
    pub page_type_offset: usize,
    /// Offset of the raw block or row-count field.
    pub block_count_offset: usize,
    /// Offset of the subheader-count field.
    pub subheader_count_offset: usize,
    /// Offset of the page-subheader-count field.
    pub page_subheader_count_offset: usize,
}
