use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformRequest {
    pub source: SourceContract,
    pub decoder: DecoderContract,
    pub transform: TransformContract,
    pub sink: SinkContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContract {
    pub path: PathBuf,
    pub format: SourceFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFormat {
    Sas7bdat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoderContract {
    pub mode: DecodeMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeMode {
    StreamingPages,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformContract {
    pub selection: Vec<String>,
    pub filter: Option<String>,
    pub execution: ExecutionModel,
    pub tuning: TransformTuning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformTuning {
    pub batch_size_rows: usize,
    pub worker_threads: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionModel {
    Streaming,
    BoundedMemory { max_rows_in_memory: usize },
}

impl ExecutionModel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::BoundedMemory { .. } => "bounded-memory",
        }
    }

    pub fn supports_larger_than_memory_inputs(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinkContract {
    pub path: PathBuf,
    pub format: SinkFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkFormat {
    Parquet,
}
