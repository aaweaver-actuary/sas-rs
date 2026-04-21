use super::{ColumnKind, ColumnMetadata, SemanticTypeHint};

/// Decoded source-column metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasColumn {
    /// Column name.
    pub name: String,
    /// Physical SAS column kind.
    pub kind: ColumnKind,
    /// Byte offset within a row.
    pub offset: usize,
    /// Physical storage width in bytes.
    pub width: usize,
    /// Semantic type inferred from SAS formats.
    pub semantic_type: SemanticTypeHint,
    /// Optional display metadata.
    pub metadata: ColumnMetadata,
}
