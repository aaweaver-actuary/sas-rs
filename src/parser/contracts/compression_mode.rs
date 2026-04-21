/// Compression mode declared by the SAS file metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMode {
    /// Uncompressed rows.
    None,
    /// Row compression.
    Row,
    /// Binary compression.
    Binary,
    /// Unrecognized compression marker.
    Unknown(u8),
}
