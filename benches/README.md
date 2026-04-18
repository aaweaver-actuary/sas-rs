# `benches`

This directory contains Criterion benchmarks and recorded real-file closure notes for the active Rust implementation.

## Benchmarks

- [assumption_probe.rs](assumption_probe.rs): a small synthetic projection benchmark used to sanity-check hot-path assumptions without involving file I/O or Parquet writing
- [parser_decode.rs](parser_decode.rs): parser-stage timing that drains `next_batch()` fully for the synthetic fixtures and retains a real streamed decode probe for `sample-sas-datasets/fts0003.sas7bdat`
- [transform_write.rs](transform_write.rs): the main end-to-end supported-subset benchmark that covers parsing, transform execution, and Parquet writing on synthetic fixtures
- [pr07_real_file_notes.md](pr07_real_file_notes.md): the authoritative PR-07 real-file CLI transform, memory, throughput, and closure notes across representative `sample-sas-datasets/` fixtures

## Which Benchmark Matters Most

For narrow supported-subset throughput experiments, `transform_write` remains the decisive synthetic end-to-end benchmark.

For request-closure evidence, pair the Criterion benches with [pr07_real_file_notes.md](pr07_real_file_notes.md). That note is the authoritative record of what the broadened real-file transform surface can do today, including the current wide-schema memory caveat.

## Caveat About `parser_decode`

`parser_decode` drains `next_batch()` until the stream is exhausted, so it is an honest streamed decode benchmark for the current supported surface.

Its real-file `fts0003` probe still matters, but PR-07 showed that parser readability alone is not enough. Final closure required the transform layer to materialize narrow numerics as well, and that path now completes end to end on `fts0003.sas7bdat`.

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

For real-file closure evidence, reproduce the release CLI commands recorded in [pr07_real_file_notes.md](pr07_real_file_notes.md).

## What Is Still Missing

This benchmark set is useful, but not universal.

Still missing:

- Criterion-style real-file transform benches checked into `cargo bench` targets
- wide-schema memory optimizations for representative real files such as `10rec.sas7bdat` and `fts0003.sas7bdat`, which now transform successfully but still peak high in RSS
- cold-cache and filesystem-sensitive benchmarks instead of the current warmed local-file measurements
