//! Contracts for representing SAS numeric values, which can be either fully parsed f64 values or
//! deferred raw bytes to be parsed later.
//!
//! The `NumericValue` enum allows for representing both parsed and unparsed numeric values, along
//! with any associated missing value tags. This is useful for handling cases where the numeric
//! value cannot be immediately parsed (e.g., due to compression or incomplete data) and needs to
//! be deferred until more information is available.
//!
//! The `Float64` variant represents a fully parsed f64 value, along with its raw bits and any
//! associated missing tag. The `DeferredBytes` variant represents raw bytes that have not yet been
//! parsed into an f64, along with the width of the numeric value in bytes. This design allows for
//! flexible handling of numeric values in various states of parsing and compression.
//!
//! # Examples
//! ```
//! use sas_rs::parser::contracts::numeric_value::NumericValue;
//! use sas_rs::parser::contracts::sas_missing_tag::SasMissingTag;
//!
//! let parsed_value = NumericValue::from(3.14);
//! assert_eq!(parsed_value.as_f64(), Some(3.14));
//! assert_eq!(parsed_value.raw_bits(), Some(3.14f64.to_bits()));
//! assert_eq!(parsed_value.missing_tag(), None);
//!
//! let deferred_value = NumericValue::deferred_bytes(vec![0x40, 0x09, 0x1E, 0xB8, 0x51, 0xEB, 0x85, 0x1F]);
//! assert_eq!(deferred_value.as_f64(), None);
//! assert_eq!(deferred_value.raw_bytes(), Some(vec![0x40, 0x09, 0x1E, 0xB8, 0x51, 0xEB, 0x85, 0x1F].as_slice()));
//! ```

use super::sas_missing_tag::SasMissingTag;

#[derive(Debug, Clone, PartialEq)]
/// Numeric cell value preserved either as a materialized float or deferred raw bytes.
pub enum NumericValue {
    /// Materialized 64-bit numeric value.
    Float64 {
        /// Decoded floating-point value.
        value: f64,
        /// Raw IEEE-754 bits used to build the value.
        raw_bits: u64,
        /// SAS special-missing tag inferred from the numeric payload.
        missing_tag: Option<SasMissingTag>,
    },
    /// Numeric bytes preserved for deferred interpretation.
    DeferredBytes {
        /// Width of the stored numeric cell in bytes.
        width_bytes: usize,
        /// Raw stored numeric bytes.
        raw_bytes: Vec<u8>,
    },
}

impl NumericValue {
    /// Build a deferred numeric value from raw bytes.
    ///
    /// # Example
    /// ```
    /// use sas_rs::parser::contracts::numeric_value::NumericValue;
    ///
    /// let raw_bytes = vec![0x40, 0x09, 0x1E, 0xB8, 0x51, 0xEB, 0x85, 0x1F];
    /// let deferred_value = NumericValue::deferred_bytes(raw_bytes.clone());
    /// assert_eq!(deferred_value.width_bytes(), 8);
    /// assert_eq!(deferred_value.raw_bytes(), Some(raw_bytes.as_slice()));
    /// ```
    pub fn deferred_bytes(raw_bytes: Vec<u8>) -> Self {
        Self::DeferredBytes {
            width_bytes: raw_bytes.len(),
            raw_bytes,
        }
    }

    /// Return the decoded float when the numeric value has been materialized.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float64 { value, .. } => Some(*value),
            Self::DeferredBytes { .. } => None,
        }
    }

    /// Return the raw IEEE-754 bits when the numeric value has been materialized.
    pub fn raw_bits(&self) -> Option<u64> {
        match self {
            Self::Float64 { raw_bits, .. } => Some(*raw_bits),
            Self::DeferredBytes { .. } => None,
        }
    }

    /// Return the stored width of the numeric cell in bytes.
    pub fn width_bytes(&self) -> usize {
        match self {
            Self::Float64 { .. } => 8,
            Self::DeferredBytes { width_bytes, .. } => *width_bytes,
        }
    }

    /// Return the deferred raw bytes when the numeric value was not materialized yet.
    pub fn raw_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Float64 { .. } => None,
            Self::DeferredBytes { raw_bytes, .. } => Some(raw_bytes.as_slice()),
        }
    }

    /// Return the inferred SAS special-missing tag when available.
    pub fn missing_tag(&self) -> Option<SasMissingTag> {
        match self {
            Self::Float64 { missing_tag, .. } => *missing_tag,
            Self::DeferredBytes { .. } => None,
        }
    }
}

impl From<f64> for NumericValue {
    fn from(value: f64) -> Self {
        Self::Float64 {
            value,
            raw_bits: value.to_bits(),
            missing_tag: None,
        }
    }
}
