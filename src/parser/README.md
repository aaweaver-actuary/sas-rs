# `src/parser`

This module owns the current `.sas7bdat` parser.

## What It Does

The parser is responsible for:

- validating the file header against the current supported subset
- decoding enough metadata to describe columns, row layout, and data pages
- exposing a `ParsedSas7bdat` handle that yields batches lazily through `next_batch`
- rejecting out-of-subset files explicitly through `UnsupportedFeature`

## Supported Parser Subset

The parser currently supports the current physical subset across these physical variants:

- word size: 32-bit and 64-bit
- endianness: little-endian and big-endian
- encoding: UTF-8, latin-1, and Windows-1252
- compression: none, row, and binary when rows are carried in the currently handled subheader path
- page types: `META`, `DATA`, and `MIX` in the combinations required by the current real-file boundary
- currently recognized subheaders:
  - row size
  - column size
  - counts
  - column text
  - column list
  - column name
  - column attributes
  - column format header shape only
- column kinds:
  - physical numeric columns with widths from 1 through 8 bytes
  - fixed-width string values decoded through the declared source encoding

8-byte numeric values currently materialize as `f64`. Narrower numeric widths are preserved as deferred raw bytes until a later semantic-typing scope decides how they should be interpreted.

## Streaming Behavior

The important parser contract is that parsing no longer means “read the whole file and build all rows first”.

The current parser:

- reads the header prefix and metadata first
- records the data-page locations
- keeps a streaming reader in `ParsedSas7bdat`
- reads additional pages only when `next_batch` is called
- stores only pending rows needed for the current batch window

That is the basis for the project’s current lazy-read and larger-than-memory claims.

## Explicitly Unsupported Parser Features

The parser currently rejects or does not implement:

- compressed page layouts or page types outside the handled `META` / `DATA` / `MIX` subset
- unknown compression modes
- subheader signatures outside the handled subset
- numeric widths greater than 8 bytes
- semantic materialization of non-8-byte numeric widths
- column type codes outside numeric and string

Real-sample evidence for the current boundary:

- `sample-sas-datasets/10rec.sas7bdat` reads through the current parser contract and exercises 64-bit big-endian plus latin-1 decoding.
- `sample-sas-datasets/fts0003.sas7bdat` now reads through the current parser contract, including row-compressed subheaders and the trailing `MIX` page used by that file.

It also does not yet expose richer SAS semantics such as:

- date/time/datetime interpretation
- labels and formats as output metadata
- special SAS missing values as first-class values

## Files

- [mod.rs](mod.rs): parser implementation and unsupported-feature handling
- [contracts.rs](contracts.rs): parser-facing contracts, streaming dataset handle, and row-batch types

## What Universal Parser Coverage Would Require

To make this parser truly universal, this module would need:

- broader page-type handling beyond the current `META` / `DATA` / `MIX` path
- more subheader coverage
- broader compression coverage beyond the current row/binary subset
- broader value and metadata interpretation
- a larger real-world fixture corpus than the synthetic subset fixtures used today