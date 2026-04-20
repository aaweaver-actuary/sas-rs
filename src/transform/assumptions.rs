//! Synthetic helpers used for small transform-planning probes.
//!
//! These types are intentionally lightweight so benchmarks and doctests can
//! exercise projection assumptions without needing a real SAS fixture.

mod build_synthetic_row_batch;
mod projection_probe_plan;
mod projection_probe_result;
mod run_projection_probe;

/// Build a synthetic row-major batch for projection probes.
pub use build_synthetic_row_batch::build_synthetic_row_batch;
/// Plan for selecting a subset of cells from a synthetic batch.
pub use projection_probe_plan::ProjectionProbePlan;
/// Result returned by a synthetic projection probe.
pub use projection_probe_result::ProjectionProbeResult;
/// Execute a synthetic projection probe and produce a deterministic checksum.
pub use run_projection_probe::run_projection_probe;
