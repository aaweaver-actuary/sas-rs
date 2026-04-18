# fuzz

This directory contains the PR-05 parser fuzzing entrypoint.

Run the parser target with `cargo-fuzz` from the repository root:

```bash
cargo fuzz run parser_entry -- -max_total_time=30
```

The target exercises the parser entrypoint and one streaming batch step so malformed inputs can shake out parse-time and first-decode panics.
