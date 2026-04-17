# `src/parser`

This module owns the current `.sas7bdat` parser.

## What It Does

The parser is responsible for:

- validating the file header against the current supported subset
- decoding enough metadata to describe columns, row layout, and data pages
- exposing a `ParsedSas7bdat` handle that yields batches lazily through `next_batch`
- rejecting out-of-subset files explicitly through `UnsupportedFeature`

## Supported Parser Subset

The parser exposes the named subset `sas7bdat-64le-uncompressed-v1`.

That means:

- word size: 64-bit only
- endianness: little-endian only
- encoding: UTF-8 only
- compression: none only
- page types: `META` and `DATA` only
- currently recognized subheaders:
  - row size
  - column size
  - column text
  - column name
  - column attributes
  - column format header shape only
- column kinds:
  - 8-byte numeric values
  - fixed-width string values

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

- 32-bit layout
- big-endian layout
- non-UTF-8 encoding codes
- row compression, binary compression, or unknown compression modes
- page types outside the handled subset
- subheader signatures outside the handled subset
- numeric widths other than 8 bytes
- column type codes outside numeric and string

It also does not yet expose richer SAS semantics such as:

- date/time/datetime interpretation
- labels and formats as output metadata
- special SAS missing values as first-class values

## Files

- [mod.rs](mod.rs): parser implementation and unsupported-feature handling
- [contracts.rs](contracts.rs): parser-facing contracts, streaming dataset handle, and row-batch types

## What Universal Parser Coverage Would Require

To make this parser truly universal, this module would need:

- 32-bit and big-endian support
- more page-type handling
- more subheader coverage
- non-UTF-8 decoding
- compression support
- broader value and metadata interpretation
- a larger real-world fixture corpus than the synthetic subset fixtures used today