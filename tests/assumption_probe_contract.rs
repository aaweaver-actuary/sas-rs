use sas_rs::transform::assumptions::{
    ProjectionProbePlan, build_synthetic_row_batch, run_projection_probe,
};

#[test]
fn synthetic_row_batches_are_row_major() {
    let batch = build_synthetic_row_batch(2, 3);

    assert_eq!(batch, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn projection_assumption_probe_reports_a_deterministic_checksum() {
    let batch = build_synthetic_row_batch(3, 4);
    let plan = ProjectionProbePlan {
        row_count: 3,
        column_count: 4,
        selected_columns: vec![1, 3],
    };

    let result = run_projection_probe(&batch, &plan);

    assert_eq!(result.selected_cells_scanned, 6);
    assert_eq!(result.checksum, 36);
}
