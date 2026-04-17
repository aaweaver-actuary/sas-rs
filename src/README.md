# `src`

This directory contains the active Rust crate that powers SAS-rs.

## Module Map

- [lib.rs](lib.rs): public crate root
- [main.rs](main.rs): thin binary entrypoint for the `sasrs` CLI
- [cli.rs](cli.rs): CLI contract and request construction
- [parser/README.md](parser/README.md): low-level `.sas7bdat` parsing and streaming row access
- [transform/README.md](transform/README.md): request contracts, execution, filtering, typing, and Parquet output

## Data Flow

The current supported-subset flow is:

1. `main.rs` forwards process arguments into `cli::run`.
2. `cli.rs` parses `transform` arguments and builds a `TransformRequest`.
3. `transform::pipeline` opens the file through a streaming source loader.
4. `parser` reads metadata, defers page reads, and yields row batches through `next_batch`.
5. `transform::sink` applies projection and filtering, converts typed batches, and writes Parquet.

## Design Intent

This crate is split so that parser, transform, and sink boundaries stay explicit.

That matters for three reasons:

- it makes unsupported subset boundaries visible instead of implicit
- it keeps streaming and bounded-memory behavior testable
- it allows the performance work to target real seams instead of one opaque code path

## What This Directory Does Not Do

This directory does not contain:

- upstream reference-reader code
- generated docs
- broad multi-format compatibility shims

Those live elsewhere or are intentionally out of scope for the current project state.