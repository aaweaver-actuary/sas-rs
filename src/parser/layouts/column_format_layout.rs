/// Offsets used when decoding a column-format subheader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnFormatLayout {
    /// Minimum accepted byte length for the subheader payload.
    pub min_len: usize,
    /// Offset of the stored format width, when present.
    pub format_width_offset: Option<usize>,
    /// Offset of the stored format precision, when present.
    pub format_digits_offset: Option<usize>,
    /// Offset of the format text reference.
    pub format_ref_offset: usize,
    /// Offset of the label text reference.
    pub label_ref_offset: usize,
}
