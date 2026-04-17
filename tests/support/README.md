# `tests/support`

This directory contains synthetic fixture helpers used by the parser and transform tests.

## What Lives Here

- [minimal_sas_fixture.rs](minimal_sas_fixture.rs):
  - synthetic `.sas7bdat` fixture builder
  - multi-page dataset generation
  - tracked readers and read-budget monitoring
  - helper paths in `.tmp/` for temporary fixture and Parquet files

## Why The Synthetic Fixture Exists

The fixture builder makes it practical to test the supported subset precisely.

It is especially useful for:

- constructing known-good parser inputs
- generating multi-page inputs for bounded-memory tests
- proving that parsing and batching begin before the full dataset is read

## What It Models

The helper models the same narrow supported subset the parser is built for:

- 64-bit little-endian layout
- UTF-8
- uncompressed pages
- numeric and fixed-width string columns
- the small set of page and subheader patterns that current tests need

## What It Does Not Replace

This helper is not a substitute for a broad real-world fixture corpus.

It does not give universal `.sas7bdat` compatibility confidence on its own. It is intentionally a controlled subset generator used to make contract, streaming, and bounded-memory tests deterministic.