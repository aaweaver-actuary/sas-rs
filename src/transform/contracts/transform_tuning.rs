/// Batch sizing and worker-tuning knobs for transform execution.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::TransformTuning;
///
/// let tuning = TransformTuning {
///     batch_size_rows: 4096,
///     worker_threads: Some(4),
/// };
///
/// assert_eq!(tuning.worker_threads, Some(4));
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformTuning {
    /// Desired row-group and decode batch size.
    pub batch_size_rows: usize,
    /// Optional explicit worker-thread count for row projection work.
    pub worker_threads: Option<usize>,
}
