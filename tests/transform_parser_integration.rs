#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use std::fs::File;
use std::io::Cursor;
use std::sync::Arc;

use arrow_array::{Array, Float64Array, StringArray};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use sas_rs::parser::{BoxedParserDataSource, SupportedSas7bdatParser};
use sas_rs::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use sas_rs::transform::pipeline::{
    ParserTransformService, SourceDataLoader, SourceDataLoaderError, TransformService,
    TransformStatus,
};
use sas_rs::transform::sink::{
    LocalParquetSink, ParquetSink, ParquetSinkError, ParquetSinkPlan, ParquetSinkReport,
    ParquetSinkStatus, StreamingParquetSink, StubParquetSink, TransformExecution,
};

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

#[derive(Debug)]
struct TrackingSourceLoader {
    bytes: Vec<u8>,
    monitor: Arc<minimal_sas_fixture::ReadMonitor>,
}

impl SourceDataLoader for TrackingSourceLoader {
    fn open(
        &self,
        _source: &SourceContract,
    ) -> Result<BoxedParserDataSource, SourceDataLoaderError> {
        Ok(Box::new(minimal_sas_fixture::tracked_reader_with_monitor(
            self.bytes.clone(),
            self.monitor.clone(),
        )))
    }
}

#[derive(Debug)]
struct FirstBatchOnlySink {
    monitor: Arc<minimal_sas_fixture::ReadMonitor>,
    max_bytes_before_first_batch: usize,
}

impl ParquetSink for FirstBatchOnlySink {
    fn prepare(&self, plan: ParquetSinkPlan) -> Result<ParquetSinkReport, ParquetSinkError> {
        Ok(ParquetSinkReport::skeleton(plan))
    }
}

impl StreamingParquetSink for FirstBatchOnlySink {
    fn stage_batches(
        &self,
        plan: ParquetSinkPlan,
        _execution: &TransformExecution,
        dataset: &mut sas_rs::parser::contracts::ParsedSas7bdat,
    ) -> Result<ParquetSinkReport, ParquetSinkError> {
        let first_batch = dataset
            .next_batch(plan.row_group_rows)
            .map_err(|error| ParquetSinkError::new(error.to_string()))?
            .ok_or_else(|| ParquetSinkError::new("expected at least one decoded batch"))?;
        let bytes_read = self.monitor.bytes_read();
        if bytes_read > self.max_bytes_before_first_batch {
            return Err(ParquetSinkError::new(format!(
                "expected first batch to stay within {} bytes before staging, read {}",
                self.max_bytes_before_first_batch, bytes_read,
            )));
        }

        Ok(ParquetSinkReport::decoded_rows_staged(
            plan,
            first_batch.rows.len(),
            1,
            0,
            1,
        ))
    }
}

#[test]
fn parser_transform_service_writes_selected_and_filtered_parquet_output() {
    let output_path =
        minimal_sas_fixture::unique_tmp_path("transform-selected-filtered", "parquet");
    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::supported_fixture_bytes(),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(example_request(output_path.clone()))
        .expect("supported fixture should transform to parquet");
    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(report.parser.subset_name, "sas7bdat-64le-uncompressed-v1");
    assert_eq!(report.parser.row_count, 3);
    assert_eq!(report.parser.column_count, 1);
    assert!(report.parser.selection_applied);
    assert!(report.parser.filter_applied);
    assert_eq!(report.sink.status, ParquetSinkStatus::ParquetWritten);
    assert_eq!(report.sink.staged_row_count, 2);
    assert_eq!(report.sink.staged_batch_count, 2);
    assert_eq!(
        read_parquet_schema(&output_path),
        vec![("customer_id".to_string(), "Float64".to_string())]
    );
    assert_eq!(read_float64_column(&output_path, 0), vec![2.5, 3.0]);
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_rejects_unsupported_filter_expressions() {
    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::supported_fixture_bytes(),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let error = service
        .run(unsupported_filter_request())
        .expect_err("unsupported filter expressions should stay explicit");
    assert!(
        error.to_string().contains("unsupported filter expression"),
        "unexpected error: {error}"
    );
}

#[test]
fn parser_transform_service_rejects_deferred_non_8_byte_numeric_materialization() {
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.columns = vec![
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "measure".to_string(),
            width: 4,
        },
        minimal_sas_fixture::FixtureColumn::String {
            name: "code".to_string(),
            width: 4,
        },
    ];
    definition.rows = vec![vec![
        minimal_sas_fixture::FixtureValue::NumericBytes(vec![0x78, 0x56, 0x34, 0x12]),
        minimal_sas_fixture::FixtureValue::String("ABCD".to_string()),
    ]];

    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, StubParquetSink);
    let error = service
        .run(deferred_numeric_request())
        .expect_err("deferred numeric materialization should stay explicit");

    assert!(
        error
            .to_string()
            .contains("numeric materialization is deferred"),
        "unexpected error: {error}"
    );
}

#[test]
fn parser_transform_service_uses_bounded_memory_batches_for_multi_page_output() {
    let output_path = minimal_sas_fixture::unique_tmp_path("transform-bounded-memory", "parquet");
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.rows = (0..700)
        .map(|index| {
            vec![
                minimal_sas_fixture::FixtureValue::Numeric(index as f64),
                minimal_sas_fixture::FixtureValue::String(format!("{:04}", index % 10_000)),
            ]
        })
        .collect();
    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(bounded_memory_request(output_path.clone()))
        .expect("multi-page fixture should write parquet in bounded batches");
    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(report.sink.status, ParquetSinkStatus::ParquetWritten);
    assert_eq!(report.sink.plan.row_group_rows, 64);
    assert_eq!(report.sink.staged_row_count, 700);
    assert_eq!(report.sink.staged_batch_count, 11);
    assert!(!report.parser.selection_applied);
    assert!(!report.parser.filter_applied);
    assert_eq!(read_total_rows(&output_path), 700);
    assert_eq!(read_float64_column(&output_path, 0)[0], 0.0);
    assert_eq!(
        read_utf8_column(&output_path, 1).last().cloned(),
        Some("0699".to_string())
    );
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_reports_parallel_batch_execution_when_worker_threads_are_used() {
    let output_path = minimal_sas_fixture::unique_tmp_path("transform-parallel-batch", "parquet");
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.rows = (0..16_384)
        .map(|index| {
            vec![
                minimal_sas_fixture::FixtureValue::Numeric(index as f64),
                minimal_sas_fixture::FixtureValue::String(format!("{:04}", index % 10_000)),
            ]
        })
        .collect();
    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(parallel_batch_request(output_path.clone()))
        .expect("large supported fixture should transform with the requested worker threads");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(report.sink.staged_row_count, 16_384);
    assert_eq!(report.sink.parallel_batch_count, 1);
    assert_eq!(report.sink.transform_threads_used, 4);
    assert_eq!(read_total_rows(&output_path), 16_384);
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_starts_batching_before_the_full_dataset_is_read() {
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.rows = (0..5_000)
        .map(|index| {
            vec![
                minimal_sas_fixture::FixtureValue::Numeric(index as f64),
                minimal_sas_fixture::FixtureValue::String(format!("{index:04}")),
            ]
        })
        .collect();

    let page_count = minimal_sas_fixture::page_count_for(&definition);
    let monitor = Arc::new(minimal_sas_fixture::ReadMonitor::default());
    let loader = TrackingSourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
        monitor: monitor.clone(),
    };
    let sink = FirstBatchOnlySink {
        monitor,
        max_bytes_before_first_batch: minimal_sas_fixture::first_batch_read_budget(page_count),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, sink);
    let report = service
        .run(first_batch_only_request())
        .expect("streaming transform should stage the first batch without whole-file reads");

    assert_eq!(report.status, TransformStatus::DecodedRowsStaged);
    assert_eq!(report.sink.status, ParquetSinkStatus::DecodedRowsStaged);
    assert_eq!(report.sink.staged_row_count, 64);
    assert_eq!(report.sink.staged_batch_count, 1);
}

fn example_request(output_path: std::path::PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/example.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: vec!["customer_id".to_string()],
            filter: Some("customer_id > 1".to_string()),
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 2,
                worker_threads: Some(2),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn unsupported_filter_request() -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/example.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: Some("customer_id > 1 AND code = EFGH".to_string()),
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 2,
                worker_threads: None,
            },
        },
        sink: SinkContract {
            path: minimal_sas_fixture::unique_tmp_path("transform-unsupported-filter", "parquet"),
            format: SinkFormat::Parquet,
        },
    }
}

fn deferred_numeric_request() -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/deferred-numeric.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: None,
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 8,
                worker_threads: Some(1),
            },
        },
        sink: SinkContract {
            path: "fixtures/deferred-numeric.parquet".into(),
            format: SinkFormat::Parquet,
        },
    }
}

fn bounded_memory_request(output_path: std::path::PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/multi-page.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: None,
            execution: ExecutionModel::BoundedMemory {
                max_rows_in_memory: 64,
            },
            tuning: TransformTuning {
                batch_size_rows: 128,
                worker_threads: Some(2),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn parallel_batch_request(output_path: std::path::PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/parallel.sas7bdat".into(),
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
                worker_threads: Some(4),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn first_batch_only_request() -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/streaming.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: Vec::new(),
            filter: None,
            execution: ExecutionModel::BoundedMemory {
                max_rows_in_memory: 64,
            },
            tuning: TransformTuning {
                batch_size_rows: 64,
                worker_threads: Some(1),
            },
        },
        sink: SinkContract {
            path: minimal_sas_fixture::unique_tmp_path("transform-first-batch-only", "parquet"),
            format: SinkFormat::Parquet,
        },
    }
}

fn read_parquet_schema(path: &std::path::Path) -> Vec<(String, String)> {
    let reader = File::open(path).expect("parquet output should exist");
    let builder =
        ParquetRecordBatchReaderBuilder::try_new(reader).expect("parquet file should be readable");
    builder
        .schema()
        .fields()
        .iter()
        .map(|field| (field.name().to_string(), format!("{:?}", field.data_type())))
        .collect()
}

fn read_total_rows(path: &std::path::Path) -> usize {
    let reader = File::open(path).expect("parquet output should exist");
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .expect("parquet file should be readable")
        .build()
        .expect("record batch reader should build");
    record_reader
        .map(|batch| batch.expect("record batch should decode").num_rows())
        .sum()
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

fn read_utf8_column(path: &std::path::Path, column_index: usize) -> Vec<String> {
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
                .downcast_ref::<StringArray>()
                .expect("column should be Utf8");
            (0..array.len())
                .map(|index| array.value(index).to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
