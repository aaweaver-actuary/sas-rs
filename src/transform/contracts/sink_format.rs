/// Output encoding supported by the public sink contract.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::SinkFormat;
///
/// assert_eq!(SinkFormat::Parquet, SinkFormat::Parquet);
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkFormat {
    /// Write transformed output as Parquet.
    Parquet,
}
