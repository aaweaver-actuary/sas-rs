//! Reviewable SAS parsing, transformation, and validation primitives.
//!
//! The crate is organized around a small set of explicit seams:
//!
//! - `parser` classifies a supported `.sas7bdat` subset and streams decoded rows.
//! - `transform` turns decoded rows into reportable execution results and Parquet output.
//! - `validation` holds the fixture-driven harnesses that keep the supported boundary honest.
//! - `cli` wires the public contracts into the `sasrs` command-line entrypoints.
//!
//! # Examples
//!
//! ```
//! use sas_rs::parser::contracts::{CompressionMode, Endianness, WordSize, supported_subset};
//!
//! let subset = supported_subset(
//!     WordSize::Bit64,
//!     Endianness::Little,
//!     CompressionMode::None,
//! );
//!
//! assert_eq!(subset.name, "sas7bdat-64le-uncompressed-v1");
//! ```

pub mod cli;
pub mod parser;
pub mod transform;
pub mod validation;
