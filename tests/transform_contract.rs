use std::path::PathBuf;

use sas_rs::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use sas_rs::transform::pipeline::{StubTransformService, TransformService, TransformStatus};
use sas_rs::transform::sink::{ParquetSink, ParquetSinkPlan, ParquetSinkStatus, StubParquetSink};

#[test]
fn stub_transform_service_returns_a_structured_not_yet_implemented_report() {
    let request = example_request();
    let service = StubTransformService::new(StubParquetSink);
    let report = service
        .run(request.clone())
        .expect("stub should always accept the request");
    assert_eq!(report.status, TransformStatus::NotYetImplemented);
    assert_eq!(report.request, request);
    assert_eq!(report.sink.status, ParquetSinkStatus::SkeletonReady);
    assert_eq!(report.sink.plan.row_group_rows, 4096);
    assert!(report.summary().contains("fixtures/example.sas7bdat"));
    assert!(report.summary().contains("fixtures/example.parquet"));
    assert!(report.summary().contains("sink_status=parquet-skeleton"));
}

#[test]
fn bounded_memory_execution_path_is_explicit_in_the_contract() {
    let execution = ExecutionModel::BoundedMemory {
        max_rows_in_memory: 1024,
    };
    assert_eq!(execution.label(), "bounded-memory");
    assert!(execution.supports_larger_than_memory_inputs());
}

#[test]
fn parquet_sink_plan_caps_row_groups_to_the_bounded_memory_budget() {
    let mut request = example_request();
    request.transform.execution = ExecutionModel::BoundedMemory {
        max_rows_in_memory: 1024,
    };
    let plan = ParquetSinkPlan::from_request(&request);
    let report = StubParquetSink
        .prepare(plan.clone())
        .expect("stub parquet sink should accept the plan");
    assert_eq!(
        plan.output_path.to_string_lossy(),
        "fixtures/example.parquet"
    );
    assert_eq!(plan.row_group_rows, 1024);
    assert_eq!(report.plan, plan);
    assert_eq!(report.status, ParquetSinkStatus::SkeletonReady);
}

fn example_request() -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: PathBuf::from("fixtures/example.sas7bdat"),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: vec!["customer_id".to_string()],
            filter: Some("customer_id > 100".to_string()),
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 4096,
                worker_threads: Some(2),
            },
        },
        sink: SinkContract {
            path: PathBuf::from("fixtures/example.parquet"),
            format: SinkFormat::Parquet,
        },
    }
}
