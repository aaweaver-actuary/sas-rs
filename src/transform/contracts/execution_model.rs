/// Execution policy for the transform layer.
///
/// The execution model controls how much decoded data may be retained while the
/// sink is being populated.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::ExecutionModel;
///
/// let model = ExecutionModel::BoundedMemory {
///     max_rows_in_memory: 2048,
/// };
///
/// assert_eq!(model.label(), "bounded-memory");
/// assert!(model.supports_larger_than_memory_inputs());
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionModel {
    /// Stream batches through the sink without a caller-specified memory cap.
    Streaming,
    /// Keep at most `max_rows_in_memory` decoded rows resident at once.
    BoundedMemory {
        /// Maximum number of decoded rows allowed in memory at once.
        max_rows_in_memory: usize,
    },
}

impl ExecutionModel {
    /// Return the stable machine-readable label for this execution model.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Streaming => "streaming",
            Self::BoundedMemory { .. } => "bounded-memory",
        }
    }

    /// Report whether this execution model can process inputs larger than RAM.
    pub fn supports_larger_than_memory_inputs(&self) -> bool {
        true
    }
}
