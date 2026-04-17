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
    ParserTransformService, SourceDataLoader, SourceDataLoaderError, TransformService,
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

criterion_group!(benches, transform_write_benchmark);
criterion_main!(benches);
