/// Sink destination and format contract.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use sas_rs::transform::contracts::{SinkContract, SinkFormat};
///
/// let sink = SinkContract {
///     path: PathBuf::from("output.parquet"),
///     format: SinkFormat::Parquet,
/// };
///
/// assert_eq!(sink.path, PathBuf::from("output.parquet"));
/// ```
use std::path::PathBuf;

use super::SinkFormat;

/// Sink destination and format contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinkContract {
    /// Filesystem path where the sink should write its output.
    pub path: PathBuf,
    /// Output encoding requested from the sink.
    pub format: SinkFormat,
}
