/// Run a deterministic projection probe over a row-major synthetic batch.
///
/// # Panics
///
/// Panics if the supplied batch shape does not match `plan`, or if a selected
/// column falls outside the declared column count.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::assumptions::{
///     ProjectionProbePlan, build_synthetic_row_batch, run_projection_probe,
/// };
///
/// let batch = build_synthetic_row_batch(2, 3);
/// let plan = ProjectionProbePlan {
///     row_count: 2,
///     column_count: 3,
///     selected_columns: vec![0, 2],
/// };
/// let result = run_projection_probe(&batch, &plan);
///
/// assert_eq!(result.selected_cells_scanned, 4);
/// assert_eq!(result.checksum, 10);
/// ```

use super::{ProjectionProbePlan, ProjectionProbeResult};

pub fn run_projection_probe(batch: &[u64], plan: &ProjectionProbePlan) -> ProjectionProbeResult {
    assert!(
        plan.column_count > 0,
        "column_count must be greater than zero"
    );
    assert_eq!(
        batch.len(),
        plan.row_count.saturating_mul(plan.column_count),
        "synthetic batch shape should match the probe plan"
    );
    assert!(
        plan.selected_columns
            .iter()
            .all(|column| *column < plan.column_count),
        "selected columns must fit inside the synthetic batch"
    );

    let checksum = batch
        .chunks_exact(plan.column_count)
        .flat_map(|row| plan.selected_columns.iter().map(move |column| row[*column]))
        .fold(0_u64, u64::wrapping_add);

    ProjectionProbeResult {
        checksum,
        selected_cells_scanned: plan.selected_cell_count(),
    }
}
