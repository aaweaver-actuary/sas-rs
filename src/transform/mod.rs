//! Transform planning, execution, and sink integration.
//!
//! The transform layer sits between streamed parser output and the final sink.
//! Its public API is intentionally contract-heavy so callers can test request
//! construction, execution planning, and sink behavior independently.

pub mod assumptions;
pub mod contracts;
pub mod pipeline;
pub mod sink;
