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

The transform layer now projects parser metadata into Arrow and Parquet as follows:

- parser `String` -> Arrow `Utf8` -> Parquet string column
- semantic numeric `Deferred` -> nullable Arrow `Float64`
- semantic numeric `Date` -> nullable Arrow `Date32`
- semantic numeric `Time` -> nullable Arrow `Time64(Microsecond)`
- semantic numeric `DateTime` -> nullable Arrow `Timestamp(Microsecond, None)`
- semantic numeric `Duration` -> nullable Arrow `Duration(Microsecond)`
- every projected numeric column also emits a nullable Utf8 sidecar column named `<column>__sas_missing_tag`

SAS labels, formats, semantic type hints, and the linked missing-tag sidecar name are preserved in Arrow field metadata when the parser surfaces them. Special missing values are represented honestly as a null in the primary typed column plus the SAS missing code in the sidecar column.

Non-8-byte numeric cells now flow through the parser contract, but the transform layer still treats their final materialization policy as deferred and fails explicitly if a batch tries to coerce them today.

## Current Filter and Projection Behavior

Selection:

- exact column-name selection only
- duplicate selected column names are rejected
- unknown selected column names are rejected
- selecting a numeric column also projects its `<column>__sas_missing_tag` sidecar

Filter expressions:

- exactly one predicate only
- token shape must be `column operator literal`
- numeric columns support `=`, `!=`, `>`, `>=`, `<`, `<=` when the parsed cell is materialized as the current numeric subset
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
- semantic materialization rules for deferred non-8-byte numeric cells
- alternative sink targets besides the current Parquet path
- guarantees that every sas7bdat informat will be present, because parser metadata is limited to what the file exposes today

## What Full Transform Coverage Would Require

To move from the current supported subset toward universal `.sas7bdat` transform coverage, this module would still need:

- richer filter and projection planning
- semantic materialization for currently deferred narrow numeric widths
- broader schema-preservation rules for metadata that are not yet surfaced by the parser
- compatibility decisions for any additional SAS-specific concepts that should appear in Arrow and Parquet
