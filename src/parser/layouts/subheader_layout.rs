/// Offsets and widths for subheader pointers under a specific layout.
/// Offsets and widths for subheader pointers under a specific layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubheaderLayout {
    /// Total byte width of one subheader pointer entry.
    /// Total byte width of one subheader pointer entry.
    pub pointer_size: usize,
    /// Offset from the pointer start to the referenced payload.
    /// Offset from the pointer start to the referenced payload.
    pub data_offset: usize,
    /// Width of the subheader signature field.
    /// Width of the subheader signature field.
    pub signature_size: usize,
    /// Width of one column-attributes entry.
    /// Width of one column-attributes entry.
    pub column_attrs_entry_size: usize,
}
