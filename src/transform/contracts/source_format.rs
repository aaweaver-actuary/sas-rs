/// Input format supported by the public source contract.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::SourceFormat;
///
/// assert_eq!(SourceFormat::Sas7bdat, SourceFormat::Sas7bdat);
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFormat {
    /// A SAS `.sas7bdat` table.
    Sas7bdat,
}
