#[path = "../tests/support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use sas_rs::parser::{ParserInput, Sas7bdatParser, SupportedSas7bdatParser};
use std::fs::File;
use std::path::PathBuf;

const BENCH_BATCH_SIZE_ROWS: usize = 8_192;

#[derive(Debug)]
enum RealFileProbeOutcome {
    Readable {
        row_count: usize,
        decoded_rows: usize,
    },
    Unsupported {
        stage: &'static str,
        detail: String,
    },
}

fn parser_decode_benchmark(criterion: &mut Criterion) {
    let parser = SupportedSas7bdatParser;
    let mut group = criterion.benchmark_group("parser_decode_supported_subset");

    for row_count in [16_384_usize, 131_072, 262_144] {
        let bytes = build_benchmark_fixture(row_count);
        group.throughput(Throughput::Elements(row_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(row_count),
            &bytes,
            |bencher, bytes| {
                bencher.iter(|| {
                    let mut parsed = parser
                        .parse(ParserInput::from_bytes(
                            "benchmark.sas7bdat",
                            black_box(bytes.clone()),
                        ))
                        .expect("benchmark fixture should parse");
                    let expected_row_count = parsed.metadata.row_count;
                    let mut decoded_rows = 0_usize;

                    while let Some(batch) = parsed
                        .next_batch(BENCH_BATCH_SIZE_ROWS)
                        .expect("benchmark fixture should stream batches")
                    {
                        decoded_rows += batch.rows.len();
                    }

                    assert_eq!(
                        decoded_rows, expected_row_count,
                        "benchmark fixture should drain the full streamed decode path"
                    );
                    black_box(decoded_rows)
                });
            },
        );
    }

    group.finish();

    let mut baseline_group = criterion.benchmark_group("parser_decode_real_file_baseline");
    baseline_group.bench_function("fts0003_probe", |bencher| {
        bencher.iter(|| match probe_fts0003_via_parser_entrypoint() {
            RealFileProbeOutcome::Readable {
                row_count,
                decoded_rows,
            } => {
                assert_eq!(
                    decoded_rows, row_count,
                    "if fts0003 becomes readable, the baseline probe should drain the full streamed decode path"
                );
                black_box(decoded_rows)
            }
            RealFileProbeOutcome::Unsupported { stage, detail } => {
                assert_eq!(
                    stage, "parse",
                    "current baseline should report the unsupported boundary at parse time"
                );
                assert_eq!(
                    detail,
                    "Unsupported(WordSize(Bit32))",
                    "current real-file baseline should stay distinct from synthetic supported-subset throughput"
                );
                black_box(detail.len())
            }
        });
    });
    baseline_group.finish();
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

fn fts0003_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sample-sas-datasets")
        .join("fts0003.sas7bdat")
}

fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {
    let path = fts0003_path();
    let path_display = path.display().to_string();
    let file = File::open(&path).expect("fts0003 fixture should be readable from disk");
    let parser = SupportedSas7bdatParser;

    let mut parsed = match parser.parse(ParserInput::from_reader(&path_display, file)) {
        Ok(parsed) => parsed,
        Err(error) => {
            return RealFileProbeOutcome::Unsupported {
                stage: "parse",
                detail: format!("{error:?}"),
            };
        }
    };

    let row_count = parsed.metadata.row_count;
    let mut decoded_rows = 0;

    loop {
        match parsed.next_batch(BENCH_BATCH_SIZE_ROWS) {
            Ok(Some(batch)) => decoded_rows += batch.rows.len(),
            Ok(None) => {
                return RealFileProbeOutcome::Readable {
                    row_count,
                    decoded_rows,
                };
            }
            Err(error) => {
                return RealFileProbeOutcome::Unsupported {
                    stage: "decode",
                    detail: format!("{error:?}"),
                };
            }
        }
    }
}

criterion_group!(benches, parser_decode_benchmark);
criterion_main!(benches);
