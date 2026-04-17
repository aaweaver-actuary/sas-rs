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
