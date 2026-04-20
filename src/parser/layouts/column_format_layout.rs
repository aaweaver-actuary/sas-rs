#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnFormatLayout {
    pub min_len: usize,
    pub format_width_offset: Option<usize>,
    pub format_digits_offset: Option<usize>,
    pub format_ref_offset: usize,
    pub label_ref_offset: usize,
}
