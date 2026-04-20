/// End-to-end request accepted by the public transform service boundary.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use sas_rs::transform::contracts::{
///     DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
///     SourceFormat, TransformContract, TransformRequest, TransformTuning,
/// };
///
/// let request = TransformRequest {
///     source: SourceContract {
///         path: PathBuf::from("input.sas7bdat"),
///         format: SourceFormat::Sas7bdat,
///     },
///     decoder: DecoderContract {
///         mode: DecodeMode::StreamingPages,
///     },
///     transform: TransformContract {
///         selection: vec!["value".to_string()],
///         filter: None,
///         execution: ExecutionModel::BoundedMemory {
///             max_rows_in_memory: 1024,
///         },
///         tuning: TransformTuning {
///             batch_size_rows: 256,
///             worker_threads: Some(2),
///         },
///     },
///     sink: SinkContract {
///         path: PathBuf::from("output.parquet"),
///         format: SinkFormat::Parquet,
///     },
/// };
///
/// assert_eq!(request.transform.tuning.batch_size_rows, 256);
/// ```

use super::{DecoderContract, SinkContract, SourceContract, TransformContract};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformRequest {
    /// Source dataset contract.
    pub source: SourceContract,
    /// Decoder settings.
    pub decoder: DecoderContract,
    /// Transform execution settings.
    pub transform: TransformContract,
    /// Sink destination contract.
    pub sink: SinkContract,
}
