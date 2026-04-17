use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use sas_rs::transform::assumptions::{
    ProjectionProbePlan, build_synthetic_row_batch, run_projection_probe,
};

fn projection_assumption_probe(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("projection_assumption_probe");

    for row_count in [16_384_usize, 131_072] {
        let plan = ProjectionProbePlan {
            row_count,
            column_count: 8,
            selected_columns: vec![0, 3, 5],
        };
        let batch = build_synthetic_row_batch(plan.row_count, plan.column_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(row_count),
            &row_count,
            |bencher, _| {
                bencher
                    .iter(|| black_box(run_projection_probe(black_box(&batch), black_box(&plan))));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, projection_assumption_probe);
criterion_main!(benches);
