#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageHeaderLayout {
    pub size: usize,
    pub page_type_offset: usize,
    pub block_count_offset: usize,
    pub subheader_count_offset: usize,
    pub page_subheader_count_offset: usize,
}
