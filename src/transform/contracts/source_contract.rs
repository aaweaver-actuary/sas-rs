/// Source dataset location and format contract.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use sas_rs::transform::contracts::{SourceContract, SourceFormat};
///
/// let source = SourceContract {
///     path: PathBuf::from("input.sas7bdat"),
///     format: SourceFormat::Sas7bdat,
/// };
///
/// assert_eq!(source.format, SourceFormat::Sas7bdat);
/// ```
use std::path::PathBuf;

use super::SourceFormat;

/// Source dataset location and format contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContract {
    /// Filesystem path from which the source loader should read.
    pub path: PathBuf,
    /// Format expected at the source path.
    pub format: SourceFormat,
}
