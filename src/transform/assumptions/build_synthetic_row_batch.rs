/// Build a synthetic row-major batch filled with monotonically increasing cell values.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::assumptions::build_synthetic_row_batch;
///
/// let batch = build_synthetic_row_batch(2, 3);
///
/// assert_eq!(batch, vec![0, 1, 2, 3, 4, 5]);
/// ```
pub fn build_synthetic_row_batch(row_count: usize, column_count: usize) -> Vec<u64> {
    (0..row_count.saturating_mul(column_count))
        .map(|index| index as u64)
        .collect()
}
