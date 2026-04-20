#[path = "../tests/support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use std::io::Cursor;
use std::path::PathBuf;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use sas_rs::parser::{BoxedParserDataSource, SupportedSas7bdatParser};
use sas_rs::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use sas_rs::transform::pipeline::{
    FileSystemSourceLoader, ParserTransformService, SourceDataLoader, SourceDataLoaderError,
    TransformService,
};
use sas_rs::transform::sink::LocalParquetSink;

#[derive(Debug)]
struct InMemorySourceLoader {
    bytes: Vec<u8>,
}

impl SourceDataLoader for InMemorySourceLoader {
    fn open(
        &self,
        _source: &SourceContract,
    ) -> Result<BoxedParserDataSource, SourceDataLoaderError> {
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}

fn transform_write_benchmark(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("transform_write_supported_subset");

    for row_count in [16_384_usize, 131_072, 262_144] {
        let bytes = build_benchmark_fixture(row_count);
        group.throughput(Throughput::Elements(row_count as u64));

        for worker_threads in [1_usize, 4] {
            let output_path = minimal_sas_fixture::unique_tmp_path(
                &format!("transform-write-bench-{row_count}-{worker_threads}"),
                "parquet",
            );
            let service = ParserTransformService::new(
                InMemorySourceLoader {
                    bytes: bytes.clone(),
                },
                SupportedSas7bdatParser,
                LocalParquetSink,
            );
            let request = bench_request(output_path.clone(), worker_threads);

            group.bench_with_input(
                BenchmarkId::new(format!("{worker_threads}-threads"), row_count),
                &request,
                |bencher, request| {
                    bencher.iter(|| {
                        let report = service
                            .run(request.clone())
                            .expect("benchmark transform should succeed");
                        black_box((
                            report.sink.output_size_bytes,
                            report.sink.parallel_batch_count,
                            report.sink.transform_threads_used,
                        ))
                    });
                },
            );

            let _ = std::fs::remove_file(output_path);
        }
    }

    group.finish();

    let mut real_group = criterion.benchmark_group("transform_write_real_file");
    for workload in real_file_workloads() {
        let request = real_file_request(&workload);
        let service = ParserTransformService::new(
            FileSystemSourceLoader,
            SupportedSas7bdatParser,
            LocalParquetSink,
        );
        real_group.bench_function(workload.benchmark_id, |bencher| {
            bencher.iter(|| {
                let report = service
                    .run(request.clone())
                    .expect("real-file transform benchmark should succeed");
                black_box((
                    report.sink.staged_row_count,
                    report.sink.staged_batch_count,
                    report.sink.output_size_bytes,
                    report.sink.parallel_batch_count,
                    report.sink.transform_threads_used,
                ))
            });
        });
        let _ = std::fs::remove_file(&workload.output_path);
    }
    real_group.finish();
}

fn bench_request(output_path: PathBuf, worker_threads: usize) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: PathBuf::from("benchmark.sas7bdat"),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: None,
            execution: ExecutionModel::BoundedMemory {
                max_rows_in_memory: 16_384,
            },
            tuning: TransformTuning {
                batch_size_rows: 16_384,
                worker_threads: Some(worker_threads),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn build_benchmark_fixture(row_count: usize) -> Vec<u8> {
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.rows = (0..row_count)
        .map(|index| {
            vec![
                minimal_sas_fixture::FixtureValue::Numeric(index as f64),
                minimal_sas_fixture::FixtureValue::String(format!("{:04}", index % 10_000)),
            ]
        })
        .collect();
    minimal_sas_fixture::build_fixture(&definition)
}

#[derive(Debug, Clone)]
struct RealFileWorkload {
    benchmark_id: &'static str,
    input_path: PathBuf,
    output_path: PathBuf,
    batch_size_rows: usize,
    max_rows_in_memory: usize,
    worker_threads: usize,
}

fn real_file_workloads() -> Vec<RealFileWorkload> {
    vec![
        RealFileWorkload {
            benchmark_id: "10rec_wide_schema_serial",
            input_path: sample_dataset_path("10rec.sas7bdat"),
            output_path: PathBuf::from(".tmp/criterion-transform-write-10rec.parquet"),
            batch_size_rows: 4,
            max_rows_in_memory: 4,
            worker_threads: 1,
        },
        RealFileWorkload {
            benchmark_id: "fts0003_wide_schema_serial",
            input_path: sample_dataset_path("fts0003.sas7bdat"),
            output_path: PathBuf::from(".tmp/criterion-transform-write-fts0003.parquet"),
            batch_size_rows: 2_048,
            max_rows_in_memory: 2_048,
            worker_threads: 1,
        },
        RealFileWorkload {
            benchmark_id: "numeric_1000000_2_serial",
            input_path: sample_dataset_path("numeric_1000000_2.sas7bdat"),
            output_path: PathBuf::from(".tmp/criterion-transform-write-numeric-1m.parquet"),
            batch_size_rows: 65_536,
            max_rows_in_memory: 65_536,
            worker_threads: 1,
        },
    ]
}

fn sample_dataset_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sample-sas-datasets")
        .join(file_name)
}

fn real_file_request(workload: &RealFileWorkload) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: workload.input_path.clone(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: None,
            execution: ExecutionModel::BoundedMemory {
                max_rows_in_memory: workload.max_rows_in_memory,
            },
            tuning: TransformTuning {
                batch_size_rows: workload.batch_size_rows,
                worker_threads: Some(workload.worker_threads),
            },
        },
        sink: SinkContract {
            path: workload.output_path.clone(),
            format: SinkFormat::Parquet,
        },
    }
}

criterion_group!(benches, transform_write_benchmark);
criterion_main!(benches);
