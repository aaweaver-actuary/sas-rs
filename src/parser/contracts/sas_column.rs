#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasColumn {
    pub name: String,
    pub kind: ColumnKind,
    pub offset: usize,
    pub width: usize,
    pub semantic_type: SemanticTypeHint,
    pub metadata: ColumnMetadata,
}
