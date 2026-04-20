/// Decode strategy requested from the parser layer.
///
/// The current supported subset only exposes page-streaming decode.
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::DecodeMode;
///
/// assert_eq!(DecodeMode::StreamingPages, DecodeMode::StreamingPages);
/// ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeMode {
    /// Stream rows page-by-page instead of materializing the full file eagerly.
    StreamingPages,
}
