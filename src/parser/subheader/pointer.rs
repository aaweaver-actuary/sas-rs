#[derive(Debug, Clone, Copy)]
struct SasSubheaderPointer {
    offset: usize,
    len: usize,
    compression: u8,
    is_compressed_data: bool,
}
