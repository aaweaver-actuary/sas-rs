# `src/transform`

This module owns the transform request model and the parser-to-Parquet execution path.

## Responsibilities

The transform layer handles:

- request contracts for source, decoder mode, transform options, and sink
- CLI-request execution through `ParserTransformService`
- selection and filter application on streamed row batches
- typed Arrow/Parquet conversion
- bounded-memory row-group planning
- optional parallel batch conversion with Rayon

## Current Type Mapping

The supported subset currently maps into Parquet as:

- parser physical numeric columns currently fall back to Arrow `Float64` only when the decoded cell materializes as an 8-byte numeric value
- parser `String` -> Arrow `Utf8` -> Parquet string column

Non-8-byte numeric cells now flow through the parser contract, but the transform layer still treats their final materialization policy as deferred and fails explicitly if a batch tries to coerce them today.

This is intentionally narrow. The transform layer does not yet interpret SAS formats or semantic numeric types.

## Current Filter and Projection Behavior

Selection:

- exact column-name selection only
- duplicate selected column names are rejected
- unknown selected column names are rejected

Filter expressions:

- exactly one predicate only
- token shape must be `column operator literal`
- numeric columns support `=`, `!=`, `>`, `>=`, `<`, `<=` when the parsed cell is materialized as the current 8-byte numeric subset
- string columns support `=` and `!=`
- compound expressions such as `AND` / `OR` are rejected explicitly

## Memory and Execution Model

The transform layer supports two execution labels:

- `Streaming`
- `BoundedMemory { max_rows_in_memory }`

In practice, the sink derives row-group size from the batch settings and bounded-memory cap, then repeatedly pulls batches from the parser instead of asking the parser to produce the whole dataset first.

Parallelism is currently batch-local:

- `worker_threads` influences batch conversion when the batch is large enough
- small batches stay serial to avoid paying thread-pool overhead for no benefit

## Files

- [contracts.rs](contracts.rs): request and execution-shape contracts
- [pipeline.rs](pipeline.rs): service orchestration and reporting
- [sink.rs](sink.rs): projection, filtering, typed conversion, Rayon-backed execution, and Parquet writing
- [assumptions.rs](assumptions.rs): small synthetic helpers for microbenchmark assumptions

## Explicitly Unsupported Transform Features

This module does not yet provide:

- a general expression engine
- multiple filter clauses
- projection rename or computed columns
- SAS date/time/datetime semantics
- SAS special missing-value preservation
- format-driven typing beyond the current numeric/string mapping
- materialization rules for deferred non-8-byte numeric cells
- alternative sink targets besides the current Parquet path

## What Full Transform Coverage Would Require

To move from the current supported subset toward universal `.sas7bdat` transform coverage, this module would need:

- semantic type mapping driven by SAS metadata, not only physical storage width
- richer filter and projection planning
- better null and missing-value semantics
- broader schema-preservation rules for labels, formats, and metadata
- compatibility decisions for how SAS-specific concepts should appear in Arrow and Parquet