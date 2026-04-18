# SAS-rs

SAS-rs is a Rust CLI and library for reading a narrow, explicit subset of `.sas7bdat` files and writing the result to Parquet.

The current implementation is optimized around an honest support boundary instead of claiming universal SAS compatibility. It is built to be fast on the subset it supports, explicit about what it rejects, and reviewable enough that the missing compatibility work is easy to see.

## Current Status

The project currently supports this end-to-end path:

- open a `.sas7bdat` file lazily through a `Read + Seek` source
- parse metadata and data pages for the supported subset
- batch rows incrementally instead of materializing the entire dataset first
- project and filter rows using the current supported transform contract
- write typed Parquet output through Arrow/Parquet crates
- parallelize batch conversion work with Rayon when the batch is large enough

The project does not currently claim universal `.sas7bdat` coverage.

## What Is Covered Today

### Parser boundary

The parser currently supports the current physical subset across these variants:

- 32-bit and 64-bit layouts
- little-endian and big-endian files
- UTF-8, latin-1, and Windows-1252 text decoding
- row-compressed and binary-compressed rows carried in subheaders
- `META`, `DATA`, and `MIX` pages encountered in the current sample-file boundary
- the currently handled subheaders:
  - row size
  - column size
  - counts
  - column text
  - column list
  - column name
  - column attributes
  - column format headers only to the extent needed to validate layout
- column kinds:
  - physical numeric columns with widths from 1 through 8 bytes
  - fixed-width string values decoded through the declared source encoding

### Transform boundary

The transform pipeline currently supports:

- `sasrs transform input.sas7bdat output.parquet`
- column selection by exact column name
- simple single-predicate filters such as `customer_id > 1`
- bounded-memory row-group writing
- streaming parser-to-sink delivery through `next_batch`
- typed Arrow/Parquet output:
  - `Numeric64 -> Float64`
  - `String -> Utf8`

### Performance and validation

The repository includes:

- unit and integration tests for CLI, parser, transform, streaming honesty, and curated real-fixture regression coverage
- real-sample parser coverage showing:
  - `sample-sas-datasets/10rec.sas7bdat` reads end to end through the current parser entrypoint
  - `sample-sas-datasets/fts0003.sas7bdat` now reads end to end through row-compressed subheaders plus the trailing `MIX` page
  - a curated regression corpus keeps real wide-row, many-page, semantic, and expected-unsupported boundary fixtures under test
- Criterion benchmarks for:
  - projection assumptions
  - parser-stage timing
  - end-to-end transform and Parquet writing

Current supported-subset synthetic benchmark evidence from the final completion pass:

- `transform_write` at 262,144 rows measured about `7.661-7.774` million rows per second with 4 worker threads
- that implies roughly `2.57-2.61` seconds for 20 million rows under those benchmark conditions

That is strong evidence for the supported synthetic workload. It is not a claim that every real-world `.sas7bdat` file with unsupported features will behave the same way.

## What Is Not Covered Today

The project intentionally rejects or does not yet model these areas:

- compressed page layouts or page types outside the current `META` / `DATA` / `MIX` subset
- unknown compression modes beyond the current row-compressed and binary-compressed coverage
- subheader signatures outside the currently recognized set
- numeric widths greater than 8 bytes
- semantic materialization of non-8-byte numeric widths
- column types beyond the current numeric/string subset
- semantic SAS typing such as:
  - dates
  - times
  - datetimes
  - richer format-driven interpretation
  - informats
  - labels as first-class output metadata
  - special numeric missing values with SAS semantics
- complex filter language such as:
  - `AND` / `OR`
  - functions
  - multi-clause expressions
  - string ordering comparisons
- broad compatibility guarantees against arbitrary production `.sas7bdat` files

## What It Would Take To Reach Truly Universal `.sas7bdat` Coverage

Universal coverage would require much more than adding a few parser branches. At a minimum it would need all of the following:

1. File layout coverage

- 32-bit and 64-bit layouts
- little-endian and big-endian variants
- all relevant header combinations seen in real SAS exports

2. Encoding coverage

- conversion for non-UTF-8 sources
- test fixtures covering common code pages and mixed-language datasets

3. Compression coverage

- row compression
- binary compression
- whatever additional compression modes appear in the wild
- benchmarks that prove decompression does not destroy throughput goals

4. Page and subheader coverage

- full page-type matrix beyond the current `META` / `DATA` subset
- broader subheader decoding, including variants the current parser treats as unsupported
- compatibility work for files whose layout differs from the narrow synthetic fixtures used today

5. Full SAS type and metadata semantics

- explicit handling for SAS dates, times, datetimes, and duration-like values
- preservation or translation of labels, formats, and informats
- honest treatment of SAS special missing values instead of flattening everything into plain `f64`
- schema decisions for how those concepts map into Arrow and Parquet

6. Real-world fixture corpus and differential validation

- a much larger corpus of real `.sas7bdat` files
- compatibility checks against SAS, `readstat`, or other trusted readers
- fuzzing and malformed-input coverage
- regression tests for edge cases such as wide rows, unusual strings, and many-page datasets

7. End-to-end memory discipline across all supported variants

- proof that lazy read and bounded-memory behavior still hold once broader format support is added
- streaming behavior that remains correct even when metadata, compression, and value decoding become more complex

8. Performance proof on realistic workloads

- real-file benchmarks, not only synthetic fixtures
- evidence that broader format coverage still keeps the project on track for the speed target
- careful profiling so compatibility work does not quietly undo the current fast path

In short: the project is now a strong supported-subset transformer, not a universal `.sas7bdat` implementation.

## CLI

Example:

```bash
cargo run -- transform input.sas7bdat output.parquet
```

Useful options:

- `--select customer_id,amount`
- `--filter 'customer_id > 1'`
- `--batch-size-rows 16384`
- `--max-rows-in-memory 16384`
- `--worker-threads 4`

## Benchmarks

Run the current benchmark set with:

```bash
cargo bench --bench assumption_probe -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
cargo bench --bench parser_decode -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
cargo bench --bench transform_write -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
```

The end-to-end benchmark that matters most for current completion claims is `transform_write`.

## Validation Harnesses

The repository now includes reviewable validation entrypoints for the real-fixture corpus boundary:

```bash
cargo test --test validation_contract
cargo run --bin sample_corpus_sweep -- --output .tmp/pr06-corpus-sweep.txt
cargo run --bin differential_validate -- --output .tmp/pr06-differential.txt
cargo fuzz run parser_entry -- -max_total_time=30
```

These harnesses are intentionally honest about the current boundary:

- the curated regression corpus mixes readable fixtures with explicitly expected-invalid fixtures so regressions stay reviewable
- `sample_corpus_sweep` records every fixture result, distinguishes expected-invalid files from true compatibility failures, and exits nonzero only when valid fixtures still fail
- `differential_validate` currently compares the supported semantic surface that is feasible against `haven`, centered on `dates.sas7bdat` and `missing_test.sas7bdat`
- the sweep and differential harnesses are evidence for requirement 27 closure; they do not imply the later performance and final request-closure scopes are complete

## Project Layout

- [src/README.md](src/README.md): crate layout and module boundaries
- [src/parser/README.md](src/parser/README.md): parser contract, supported subset, and parser-specific limits
- [src/transform/README.md](src/transform/README.md): transform, filter, typing, sink, and batching behavior
- [benches/README.md](benches/README.md): benchmark intent and interpretation notes
- [tests/README.md](tests/README.md): test organization and what each suite proves
- [tests/support/README.md](tests/support/README.md): synthetic fixture generator and read-budget helpers

Reference codebases bundled in the repository:

- `readstat-example/` for C/reference behavior and fixture ideas
- `haven-example/` for R-facing semantics and compatibility context