# PR-07 Real-File Notes

Date: 2026-04-17
Environment: local macOS workspace, `cargo build --release`, release CLI at `target/release/sasrs`

## Exact Commands

- `target/release/sasrs transform sample-sas-datasets/dates.sas7bdat .tmp/pr07-dates.parquet --select dt,dates,times --filter 'dates >= 0' --batch-size-rows 2 --max-rows-in-memory 2 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/issue_pandas.sas7bdat .tmp/pr07-issue-pandas.parquet --batch-size-rows 2 --max-rows-in-memory 2 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/sample_bincompressed.sas7bdat .tmp/pr07-bincompressed.parquet --batch-size-rows 2 --max-rows-in-memory 2 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/10rec.sas7bdat .tmp/pr07-10rec.parquet --batch-size-rows 4 --max-rows-in-memory 4 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/fts0003.sas7bdat .tmp/pr07-fts0003.parquet --batch-size-rows 2048 --max-rows-in-memory 2048 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/numeric_1000000_2.sas7bdat .tmp/pr07-numeric-1m-serial.parquet --batch-size-rows 65536 --max-rows-in-memory 65536 --worker-threads 1`
- `target/release/sasrs transform sample-sas-datasets/numeric_1000000_2.sas7bdat .tmp/pr07-numeric-1m-parallel.parquet --batch-size-rows 65536 --max-rows-in-memory 65536 --worker-threads 4`

The raw parsed outputs are stored in `.tmp/pr07-real-cli-benchmarks.json` and `.tmp/pr07-real-cli-benchmarks.md`.

## Results

- `dates.sas7bdat`: pass. `parsed_rows=19`, `staged_rows=15`, `row_group_rows=2`, `selection_applied=applied`, `filter_applied=applied`, `0.00 real`, max resident set size `8486912`.
- `issue_pandas.sas7bdat`: pass. `parser_subset=sas7bdat-32le-row-compressed-v1`, `staged_rows=7`, `staged_batches=4`, `0.00 real`, max resident set size `7716864`.
- `sample_bincompressed.sas7bdat`: pass. `parser_subset=sas7bdat-64le-binary-compressed-v1`, `staged_rows=5`, `staged_batches=3`, `0.00 real`, max resident set size `8650752`.
- `10rec.sas7bdat`: pass. `parser_subset=sas7bdat-64be-uncompressed-v1`, `staged_rows=10`, `staged_batches=3`, `row_group_rows=4`, `0.92 real`, max resident set size `1150599168`.
- `fts0003.sas7bdat`: pass. `parser_subset=sas7bdat-32le-binary-compressed-v1`, `staged_rows=10275`, `staged_batches=6`, `row_group_rows=2048`, `2.65 real`, max resident set size `912687104`.
- `numeric_1000000_2.sas7bdat` serial: pass. `parsed_rows=1000000`, `staged_rows=1000000`, `row_group_rows=65536`, `parallel_batches=0`, `transform_threads_used=1`, `0.14 real`, max resident set size `24756224`.
- `numeric_1000000_2.sas7bdat` parallel: pass. `parsed_rows=1000000`, `staged_rows=1000000`, `row_group_rows=65536`, `parallel_batches=16`, `transform_threads_used=4`, `0.09 real`, max resident set size `36945920`.

## Findings

- The transform layer now materializes 1-byte through 7-byte SAS numerics with the same endianness-aware byte reconstruction used by established readers, and it preserves SAS special missing tags on that path instead of flattening them silently.
- The previously blocked representative fixtures now close honestly: `10rec.sas7bdat` and `fts0003.sas7bdat` both write parquet end to end under the release CLI.
- Bounded-memory execution remains concrete on the broadened real surface because every successful run above used `ExecutionModel::BoundedMemory`, and the row-group caps stayed at `2`, `2`, `2`, `4`, `2048`, and `65536` rows respectively.
- Very wide real files still carry substantial schema and column-buffer overhead. `10rec.sas7bdat` peaked near `1.15 GB` RSS and `fts0003.sas7bdat` peaked near `913 MB` RSS even though row-group staging remained bounded. This is an optimization follow-up, not the current closure blocker.
- Rayon-backed execution remains concrete on representative real data: the million-row workload moved from `0.14 real` serial to `0.09 real` with `parallel_batches=16` and `transform_threads_used=4`.
- The warmed local million-row measurement is comfortably above the request bar for `~20M` rows in under a minute even under a conservative linear extrapolation, while the wide-schema and big-endian real fixtures now also complete end to end on the broadened supported surface.
- Issue `#7` remains a legitimate optimization idea, but the final request is no longer blocked on missing transform support.

## Closure Verdict

PR-07 is ready for final review. The remaining work is optimization follow-up only: most notably reducing peak RSS on extremely wide real files and adding first-class real-file Criterion benches if those measurements need to live directly under `cargo bench`.
