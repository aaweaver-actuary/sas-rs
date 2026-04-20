/// Selection plan for a synthetic projection probe.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::assumptions::ProjectionProbePlan;
///
/// let plan = ProjectionProbePlan {
///     row_count: 4,
///     column_count: 3,
///     selected_columns: vec![0, 2],
/// };
///
/// assert_eq!(plan.selected_cell_count(), 8);
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionProbePlan {
    /// Number of logical rows in the synthetic batch.
    pub row_count: usize,
    /// Number of columns per row in the synthetic batch.
    pub column_count: usize,
    /// Zero-based columns to include in the probe checksum.
    pub selected_columns: Vec<usize>,
}

impl ProjectionProbePlan {
    /// Return how many cells the selection will scan if every row is visited.
    pub fn selected_cell_count(&self) -> usize {
        self.row_count.saturating_mul(self.selected_columns.len())
    }
}
