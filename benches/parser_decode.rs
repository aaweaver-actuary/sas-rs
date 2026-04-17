#[path = "../tests/support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use sas_rs::parser::{ParserInput, Sas7bdatParser, SupportedSas7bdatParser};

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
                    let parsed = parser
                        .parse(ParserInput::from_bytes(
                            "benchmark.sas7bdat",
                            black_box(bytes.clone()),
                        ))
                        .expect("benchmark fixture should parse");
                    black_box(parsed.metadata.row_count)
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
