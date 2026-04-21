#[derive(Debug, Clone, Copy)]
pub(super) struct SasSubheaderPointer {
    pub(super) offset: usize,
    pub(super) len: usize,
    pub(super) compression: u8,
    pub(super) is_compressed_data: bool,
}
