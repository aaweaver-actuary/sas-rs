#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionProbePlan {
    pub row_count: usize,
    pub column_count: usize,
    pub selected_columns: Vec<usize>,
}

impl ProjectionProbePlan {
    pub fn selected_cell_count(&self) -> usize {
        self.row_count.saturating_mul(self.selected_columns.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionProbeResult {
    pub checksum: u64,
    pub selected_cells_scanned: usize,
}

pub fn build_synthetic_row_batch(row_count: usize, column_count: usize) -> Vec<u64> {
    (0..row_count.saturating_mul(column_count))
        .map(|index| index as u64)
        .collect()
}

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
