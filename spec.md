# SAS-rs: The fastest `.sas7bdat` reader _by far_

The goal of SAS-rs is to be the fastest `.sas7bdat` file reader that exists today. Even faster than SAS itself.

This will be accomplished by:

1. Iteration
  - A "complete" or "functional" reader is not a goal of this project, but merely a byproduct of our relentless pursuit of performance
2. Imagination
  - We prioritize out-of-the-box thinking over dogma, and are not afraid to run expirements
3. Rigor
  - We keep meticulous notes in a journal documenting each expirement we do, whether it improved performance or not, and potential takaways for the next expirement

## Requirements

1. Lazily read sas7bdat files, decode the SAS binary format, re-save as parquet file
2. CLI to efficiently transform files
3. Takes advantage of rust's parallel processing with rayon, as well as modern conveniences like zero-cost abstractions and function in-lining when appropriate
4. Should be able to transform ~20M rows in < 1 minute
5. Clear and human-readable rust implementation
6. Fully-unit-tested
7. Performance benchmarking at each useful point
8. Idiomatic rust conventions are always used
9. Able to handle larger-than-memory datasets
10. CLI includes options to select or filter columns, as well as any tuning parameters we need
11. Uses crates from polars or similar to automatically type the data appropriately before saving to parquet
12. Coverage for both 32-bit and 64-bit layouts
13. Coverage for both little-endian and big-endian variants
14. Coverage for non-UTF-8 encoding codes, in particular latin-1, which is common in SAS exports
15. Coverage for row compression
16. Coverage for binary compression
17. Coverage for all page types, including `META`, `DATA`, and any others we encounter in the wild
18. Coverage for all relevant subheader signatures, including those for row size
19. Coverage for all numeric widths, not just 8-byte values
20. Coverage for SAS dates, times, datetimes, and duration-like values
21. Translation of SAS labels, formats, and informats into appropriate Arrow and Parquet metadata
22. Honest treatment of SAS special missing values instead of flattening everything into plain `f64`
23. Fuzzing and malformed-input coverage
24. Regression tests for edge cases such as wide rows, unusual strings, and many-page datasets
25. Proof that lazy read and bounded-memory behavior still hold once broader format support is added
26. Proof that streaming behavior remains correct even when metadata, compression, and value decoding become more complex
27. All datasets inside the `sample-sas-datasets` directory are readable by the final implementation
28. Benchmarking is done on representative datasets from the `sample-sas-datasets` directory, and the results are documented in the journal
  - `fts0003.sas7bdat` is required to be part of the benchmark suite. While it is not "real" in the sense of measuring anything in particular, it is often used as a benchmark in other implementations, since it has > 1000 columns and > 1M rows, making it a good test of performance on wide and large datasets
  - several other datasets from the `sample-sas-datasets` directory are also included in the benchmark suite, to ensure that we are not overfitting to a single dataset, and are representative of the variety of data types, encodings, formats, and compression schemes we encounter in the wild

## Starting point

### `readstat`

One of the only C implementations I am aware of. Extremely useful as a starting point, though keep in mind that some C idioms will not always translate to rust perfectly, and idiomatic code is more important than 100% fidelity. Ultimately we are trying to get much faster than this existing implementation. 

### `haven` 

An implementation for R that relies heavily on a version of readstat. 

## Expected user workflow

```bash
sasrs transform my-sas-file.sas7bdat my-parquet-file.parquet
```

Within a minute or so, the file has been transformed and given appropriate data types to the columns.
