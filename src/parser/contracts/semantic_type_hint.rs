/// Semantic type inferred from SAS formatting metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTypeHint {
    /// No stronger semantic type was inferred.
    Deferred,
    /// SAS date value.
    Date,
    /// SAS time value.
    Time,
    /// SAS datetime value.
    DateTime,
    /// SAS duration value.
    Duration,
}

impl SemanticTypeHint {
    /// Return the stable machine-readable label for this semantic type.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime",
            Self::Duration => "duration",
        }
    }
}
