# `benches`

This directory contains Criterion benchmarks for the active Rust implementation.

## Benchmarks

- [assumption_probe.rs](assumption_probe.rs): a small synthetic projection benchmark used to sanity-check hot-path assumptions without involving file I/O or Parquet writing
- [parser_decode.rs](parser_decode.rs): parser-stage timing that now drains `next_batch()` fully for the current supported subset, so it measures parse setup plus streamed row decode on the synthetic benchmark fixtures
- [transform_write.rs](transform_write.rs): the main end-to-end supported-subset benchmark that covers parsing, transform execution, and Parquet writing

## Which Benchmark Matters Most

For current project-level performance claims, `transform_write` is the decisive benchmark.

That is the benchmark used to support the current supported-subset throughput case.

## Caveat About `parser_decode`

`parser_decode` now drains `next_batch()` until the stream is exhausted, so it is an honest streamed decode benchmark for the current synthetic supported subset.

It still does not prove broader format compatibility or real-world filesystem behavior on its own.

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

- a corpus of real-world `.sas7bdat` files with broader compatibility coverage
- I/O-sensitive benchmarks on actual filesystems instead of synthetic in-memory fixtures only