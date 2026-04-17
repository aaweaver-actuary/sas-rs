# How do we run the foundation assumption probe benchmark?

## Answer
- Use `cargo bench --bench assumption_probe -- --noplot --sample-size 10 --measurement-time 0.2 --warm-up-time 0.1` for a quick validation run.
- The harness lives in `benches/assumption_probe.rs` and uses Criterion.
- It benchmarks `run_projection_probe` over synthetic row-major batches at 16,384 and 131,072 rows with 8 columns and 3 selected columns.
- Foundation-scope output on 2026-04-16 was about 21.9 us for 16,384 rows and 176.3 us for 131,072 rows.

## Freshness
- Status: verified against repository
- Last verified: 2026-04-16
- Verified from:
  - Cargo.toml
  - benches/assumption_probe.rs
  - cargo bench --bench assumption_probe -- --noplot --sample-size 10 --measurement-time 0.2 --warm-up-time 0.1
- Refresh when:
  - benchmark target names, probe inputs, or Criterion settings change
  - the assumption probe stops targeting the synthetic projection path
