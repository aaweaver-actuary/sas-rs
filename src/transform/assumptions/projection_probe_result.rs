/// Deterministic result produced by a synthetic projection probe.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionProbeResult {
    /// Wrapping checksum of all selected cells.
    pub checksum: u64,
    /// Count of selected cells scanned while producing the checksum.
    pub selected_cells_scanned: usize,
}
