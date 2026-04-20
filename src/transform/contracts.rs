//! Stable request and contract types for the transform layer.
//!
//! These types describe what to read, how to decode it, what transform policy
//! to apply, and where to write the result.

mod decode_mode;
mod decoder_contract;
mod execution_model;
mod sink_contract;
mod sink_format;
mod source_contract;
mod source_format;
mod transform_contract;
mod transform_request;
mod transform_tuning;

/// Decoder mode for the supported SAS parser integration.
pub use decode_mode::DecodeMode;
/// Decoder-specific request contract.
pub use decoder_contract::DecoderContract;
/// Requested execution policy for the transform phase.
pub use execution_model::ExecutionModel;
/// Sink destination contract.
pub use sink_contract::SinkContract;
/// Output format requested from the sink.
pub use sink_format::SinkFormat;
/// Source dataset location contract.
pub use source_contract::SourceContract;
/// Input format requested from the source loader.
pub use source_format::SourceFormat;
/// Transform-time projection, filter, and execution settings.
pub use transform_contract::TransformContract;
/// Full transform request passed through the service boundary.
pub use transform_request::TransformRequest;
/// Tuning knobs for batch sizing and worker parallelism.
pub use transform_tuning::TransformTuning;
