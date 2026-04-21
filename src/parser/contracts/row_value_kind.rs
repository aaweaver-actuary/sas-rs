use super::{ColumnKind, SasColumn, SemanticTypeHint};

/// Logical value kind emitted in decoded rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowValueKind {
    /// Numeric value.
    Numeric,
    /// Character value.
    Character,
    /// Date value.
    Date,
    /// Time value.
    Time,
    /// Datetime value.
    DateTime,
    /// Duration value.
    Duration,
}

impl RowValueKind {
    /// Derive the emitted row value kind from a source SAS column.
    pub fn from_source_column(column: &SasColumn) -> Self {
        match column.kind {
            ColumnKind::String => Self::Character,
            ColumnKind::Numeric => match column.semantic_type {
                SemanticTypeHint::Deferred => Self::Numeric,
                SemanticTypeHint::Date => Self::Date,
                SemanticTypeHint::Time => Self::Time,
                SemanticTypeHint::DateTime => Self::DateTime,
                SemanticTypeHint::Duration => Self::Duration,
            },
        }
    }

    /// Report whether values of this kind may be null in the output schema.
    pub fn is_nullable(self) -> bool {
        !matches!(self, Self::Character)
    }
}
