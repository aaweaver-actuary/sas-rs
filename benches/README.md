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

## PR-01 Measurement Contract

The 2026-04-22 performance campaign starts from measurement, not intuition. The current contract for optimization scopes is:

- keep `fts0003.sas7bdat` in every representative benchmark set
- pair `fts0003` with a wide fixed-cost fixture and a parallel-sensitive fixture; the current contract uses `10rec.sas7bdat` and `numeric_1000000_2.sas7bdat`
- run `assumption_probe`, `parser_decode`, and `transform_write` before changing runtime behavior so parser-only, materialization, and sink cost stay separable
- use repeated Criterion runs for parser-only and end-to-end paths, then supplement them with release-style CLI timing when a real-file benchmark needs wall time or RSS context
- check for `perf`, `cargo-flamegraph`, or `samply` before claiming profiler evidence; if they are unavailable, record that limitation explicitly and continue with repeated benchmark evidence plus code-path analysis

Current pr01 findings from this worktree:

- `parser_decode_real_file_baseline/fts0003_probe` measured `[1.2272 s 1.2790 s 1.3459 s]`
- `transform_write_real_file/fts0003_wide_schema_serial` measured `[5.5549 s 5.7019 s 5.9262 s]`
- `transform_write_real_file/10rec_wide_schema_serial` measured `[988.47 ms 1.0558 s 1.1385 s]`
- `transform_write_real_file/numeric_1000000_2_serial` measured `[288.40 ms 296.61 ms 307.39 ms]`
- release CLI timing on `fts0003` was `8.21 s` wall time with `529364 kB` max RSS
- release CLI timing on `numeric_1000000_2` was neutral in this environment: `0.50 s` serial versus `0.51 s` with four workers

## What Is Still Missing

This benchmark set is useful, but not universal.

Still missing:

- Criterion-style real-file transform benches checked into `cargo bench` targets
- wide-schema memory optimizations for representative real files such as `10rec.sas7bdat` and `fts0003.sas7bdat`, which now transform successfully but still peak high in RSS
- cold-cache and filesystem-sensitive benchmarks instead of the current warmed local-file measurements
