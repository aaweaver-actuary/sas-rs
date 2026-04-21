/// Parser-decoder settings carried inside a [`super::TransformRequest`].
///
/// # Examples
///
/// ```
/// use sas_rs::transform::contracts::{DecodeMode, DecoderContract};
///
/// let decoder = DecoderContract {
///     mode: DecodeMode::StreamingPages,
/// };
///
/// assert_eq!(decoder.mode, DecodeMode::StreamingPages);
/// ```
use super::DecodeMode;

/// Parser-decoder settings carried inside a transform request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoderContract {
    /// Decode mode the parser should use for the source file.
    pub mode: DecodeMode,
}
