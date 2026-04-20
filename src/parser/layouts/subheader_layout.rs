#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubheaderLayout {
    pub pointer_size: usize,
    pub data_offset: usize,
    pub signature_size: usize,
    pub column_attrs_entry_size: usize,
}
