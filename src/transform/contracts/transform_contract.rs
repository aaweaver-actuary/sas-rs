/// Transform-time projection, filtering, and execution settings.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::{ExecutionModel, TransformContract, TransformTuning};
///
/// let transform = TransformContract {
///     selection: vec!["id".to_string(), "name".to_string()],
///     filter: Some("id >= 10".to_string()),
///     execution: ExecutionModel::Streaming,
///     tuning: TransformTuning {
///         batch_size_rows: 512,
///         worker_threads: Some(2),
///     },
/// };
///
/// assert_eq!(transform.selection.len(), 2);
/// assert_eq!(transform.execution.label(), "streaming");
/// ```

use super::{ExecutionModel, TransformTuning};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformContract {
    /// Optional explicit column projection. An empty list means "all columns".
    pub selection: Vec<String>,
    /// Optional filter expression evaluated by the transform layer.
    pub filter: Option<String>,
    /// Requested execution policy.
    pub execution: ExecutionModel,
    /// Batch sizing and worker-tuning hints.
    pub tuning: TransformTuning,
}
