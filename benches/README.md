# `benches`

This directory contains Criterion benchmarks for the active Rust implementation.

## Benchmarks

- [assumption_probe.rs](assumption_probe.rs): a small synthetic projection benchmark used to sanity-check hot-path assumptions without involving file I/O or Parquet writing
- [parser_decode.rs](parser_decode.rs): parser-stage timing; currently useful mainly as a parser-entry benchmark, not as a full streaming row-decode benchmark
- [transform_write.rs](transform_write.rs): the main end-to-end supported-subset benchmark that covers parsing, transform execution, and Parquet writing

## Which Benchmark Matters Most

For current project-level performance claims, `transform_write` is the decisive benchmark.

That is the benchmark used to support the current supported-subset throughput case.

## Caveat About `parser_decode`

`parser_decode` currently calls `parse()` but does not drain batches with `next_batch()`.

That means it is best interpreted as parser-entry and metadata/setup timing, not a full row-decode throughput measurement. The project-level README treats it that way deliberately.

## Running The Benchmarks

```bash
cargo bench --bench assumption_probe -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
cargo bench --bench parser_decode -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
cargo bench --bench transform_write -- --noplot --sample-size 15 --measurement-time 0.5 --warm-up-time 0.2
```

For a tighter supported-subset throughput check, rerun the largest transform workload directly:

```bash
cargo bench --bench transform_write 262144 -- --noplot --sample-size 20 --measurement-time 1 --warm-up-time 0.3
```

## What Is Still Missing

This benchmark set is intentionally useful, but not universal.

Still missing:

- a parser benchmark that drains streamed batches fully
- a corpus of real-world `.sas7bdat` files with broader compatibility coverage
- I/O-sensitive benchmarks on actual filesystems instead of synthetic in-memory fixtures only