#[path = "../tests/support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use sas_rs::parser::{ParserInput, Sas7bdatParser, SupportedSas7bdatParser};

const BENCH_BATCH_SIZE_ROWS: usize = 8_192;

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

criterion_group!(benches, parser_decode_benchmark);
criterion_main!(benches);
