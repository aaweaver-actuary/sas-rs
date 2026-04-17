#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use std::cell::RefCell;
use std::fs::File;

use arrow_array::{Array, Float64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use sas_rs::cli::{CommandOutcome, run, run_with_service};
use sas_rs::transform::contracts::{
    DecodeMode, ExecutionModel, SinkFormat, SourceFormat, TransformRequest,
};
use sas_rs::transform::pipeline::{
    TransformReport, TransformService, TransformServiceError, TransformStatus,
};
use sas_rs::transform::sink::ParquetSinkStatus;

#[derive(Debug)]
struct RecordingService {
    requests: RefCell<Vec<TransformRequest>>,
}

impl RecordingService {
    fn new() -> Self {
        Self {
            requests: RefCell::new(Vec::new()),
        }
    }

    fn single_request(&self) -> TransformRequest {
        self.requests.borrow().first().cloned().expect("request")
    }
}

impl TransformService for RecordingService {
    fn run(&self, request: TransformRequest) -> Result<TransformReport, TransformServiceError> {
        self.requests.borrow_mut().push(request.clone());
        Ok(TransformReport::not_yet_implemented(request))
    }
}

#[test]
fn transform_command_builds_the_reviewable_stub_request() {
    let service = RecordingService::new();
    let outcome = run_with_service(
        [
            "sasrs",
            "transform",
            "fixtures/example.sas7bdat",
            "fixtures/example.parquet",
            "--select",
            "customer_id,amount",
            "--filter",
            "amount > 10",
            "--batch-size-rows",
            "2048",
            "--max-rows-in-memory",
            "512",
            "--worker-threads",
            "4",
        ],
        &service,
    )
    .expect("cli should parse");
    let request = service.single_request();
    assert_eq!(request.source.format, SourceFormat::Sas7bdat);
    assert_eq!(
        request.source.path.to_string_lossy(),
        "fixtures/example.sas7bdat"
    );
    assert_eq!(request.decoder.mode, DecodeMode::StreamingPages);
    assert_eq!(request.transform.selection, vec!["customer_id", "amount"]);
    assert_eq!(request.transform.filter.as_deref(), Some("amount > 10"));
    assert_eq!(request.transform.tuning.batch_size_rows, 2048);
    assert_eq!(request.transform.tuning.worker_threads, Some(4));
    assert_eq!(
        request.transform.execution,
        ExecutionModel::BoundedMemory {
            max_rows_in_memory: 512
        }
    );
    assert_eq!(request.sink.format, SinkFormat::Parquet);
    assert_eq!(
        request.sink.path.to_string_lossy(),
        "fixtures/example.parquet"
    );
    let CommandOutcome::Transform(report) = outcome;
    assert_eq!(report.status, TransformStatus::NotYetImplemented);
    assert_eq!(report.sink.status, ParquetSinkStatus::SkeletonReady);
    assert_eq!(report.sink.plan.row_group_rows, 512);
    assert!(report.summary().contains("status=not-yet-implemented"));
    assert!(report.summary().contains("sink_status=parquet-skeleton"));
}

#[test]
fn transform_command_writes_a_real_parquet_file_through_the_default_service() {
    let input_path = minimal_sas_fixture::unique_tmp_path("cli-transform-input", "sas7bdat");
    let output_path = minimal_sas_fixture::unique_tmp_path("cli-transform-output", "parquet");
    minimal_sas_fixture::write_fixture_file(
        &minimal_sas_fixture::supported_fixture_definition(),
        &input_path,
    );
    let outcome = run([
        "sasrs",
        "transform",
        input_path.to_str().expect("input path should be utf-8"),
        output_path.to_str().expect("output path should be utf-8"),
        "--select",
        "customer_id",
        "--filter",
        "customer_id > 1",
        "--batch-size-rows",
        "2",
    ])
    .expect("the default cli service should write parquet");
    let CommandOutcome::Transform(report) = outcome;
    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(report.sink.status, ParquetSinkStatus::ParquetWritten);
    assert_eq!(read_float64_column(&output_path, 0), vec![2.5, 3.0]);
    let _ = std::fs::remove_file(input_path);
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn transform_command_defaults_to_the_streaming_execution_path() {
    let service = RecordingService::new();
    run_with_service(
        [
            "sasrs",
            "transform",
            "fixtures/example.sas7bdat",
            "fixtures/example.parquet",
        ],
        &service,
    )
    .expect("cli should parse");
    let request = service.single_request();
    assert_eq!(request.transform.execution, ExecutionModel::Streaming);
    assert!(request.transform.selection.is_empty());
    assert_eq!(request.transform.filter, None);
}

fn read_float64_column(path: &std::path::Path, column_index: usize) -> Vec<f64> {
    let reader = File::open(path).expect("parquet output should exist");
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .expect("parquet file should be readable")
        .build()
        .expect("record batch reader should build");
    record_reader
        .flat_map(|batch| {
            let batch = batch.expect("record batch should decode");
            let array = batch
                .column(column_index)
                .as_any()
                .downcast_ref::<Float64Array>()
                .expect("column should be Float64");
            (0..array.len())
                .map(|index| array.value(index))
                .collect::<Vec<_>>()
        })
        .collect()
}
