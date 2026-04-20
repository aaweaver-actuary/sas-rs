#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticTypeHint {
    Deferred,
    Date,
    Time,
    DateTime,
    Duration,
}

impl SemanticTypeHint {
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

