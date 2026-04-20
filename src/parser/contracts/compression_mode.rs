#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    None,
    Row,
    Binary,
    Unknown(u8),
}
