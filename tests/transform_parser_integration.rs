#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::{
    Array, Date32Array, DurationMicrosecondArray, Float64Array, StringArray,
    Time64MicrosecondArray, TimestampMicrosecondArray,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use sas_rs::parser::contracts::Endianness;
use sas_rs::parser::{BoxedParserDataSource, SupportedSas7bdatParser};
use sas_rs::transform::contracts::{
    DecodeMode, DecoderContract, ExecutionModel, SinkContract, SinkFormat, SourceContract,
    SourceFormat, TransformContract, TransformRequest, TransformTuning,
};
use sas_rs::transform::pipeline::{
    FileSystemSourceLoader, ParserTransformService, SourceDataLoader, SourceDataLoaderError,
    TransformService, TransformStatus,
};
use sas_rs::transform::sink::{
    LocalParquetSink, ParquetSink, ParquetSinkError, ParquetSinkPlan, ParquetSinkReport,
    ParquetSinkStatus, StreamingParquetSink, TransformExecution,
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
    assert_eq!(report.parser.column_count, 2);
    assert!(report.parser.selection_applied);
    assert!(report.parser.filter_applied);
    assert_eq!(report.sink.status, ParquetSinkStatus::ParquetWritten);
    assert_eq!(report.sink.staged_row_count, 2);
    assert_eq!(report.sink.staged_batch_count, 2);
    assert_eq!(
        read_parquet_schema(&output_path),
        vec![
            ("customer_id".to_string(), "Float64".to_string()),
            (
                "customer_id__sas_missing_tag".to_string(),
                "Utf8".to_string()
            ),
        ]
    );
    assert_eq!(read_float64_column(&output_path, 0), vec![2.5, 3.0]);
    assert_eq!(read_optional_utf8_column(&output_path, 1), vec![None, None]);
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
fn parser_transform_service_materializes_non_8_byte_numeric_columns_with_filtering() {
    let output_path =
        minimal_sas_fixture::unique_tmp_path("transform-narrow-numeric-filtered", "parquet");
    let layout = minimal_sas_fixture::FixtureLayout::bit64_little();
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.layout = layout;
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
    definition.rows = vec![
        vec![
            minimal_sas_fixture::FixtureValue::NumericBytes(truncated_numeric_bytes(
                layout, 42.5, 4,
            )),
            minimal_sas_fixture::FixtureValue::String("ABCD".to_string()),
        ],
        vec![
            minimal_sas_fixture::FixtureValue::NumericBytes(truncated_numeric_bytes(
                layout, -3.25, 4,
            )),
            minimal_sas_fixture::FixtureValue::String("WXYZ".to_string()),
        ],
    ];

    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(narrow_numeric_filtered_request(output_path.clone()))
        .expect("narrow numeric materialization should support filtered parquet output");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(report.sink.status, ParquetSinkStatus::ParquetWritten);
    assert_eq!(
        read_optional_float64_column(&output_path, 0),
        vec![Some(42.5)]
    );
    assert_eq!(read_optional_utf8_column(&output_path, 1), vec![None]);
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_materializes_big_endian_non_8_byte_missing_tags() {
    let output_path =
        minimal_sas_fixture::unique_tmp_path("transform-big-endian-narrow-numeric", "parquet");
    let layout = minimal_sas_fixture::FixtureLayout::bit64_big();
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.layout = layout;
    definition.columns = vec![
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "measure".to_string(),
            width: 5,
        },
        minimal_sas_fixture::FixtureColumn::String {
            name: "code".to_string(),
            width: 4,
        },
    ];
    definition.rows = vec![
        vec![
            minimal_sas_fixture::FixtureValue::NumericBytes(truncated_numeric_bytes(
                layout, 7.0, 5,
            )),
            minimal_sas_fixture::FixtureValue::String("ABCD".to_string()),
        ],
        vec![
            minimal_sas_fixture::FixtureValue::NumericBytes(truncated_missing_numeric_bytes(
                layout, 'A', 5,
            )),
            minimal_sas_fixture::FixtureValue::String("MISS".to_string()),
        ],
    ];

    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(narrow_numeric_request(output_path.clone()))
        .expect("big-endian narrow numerics should materialize into parquet output");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(
        read_optional_float64_column(&output_path, 0),
        vec![Some(7.0), None]
    );
    assert_eq!(
        read_optional_utf8_column(&output_path, 1),
        vec![None, Some("A".to_string())]
    );
    let _ = std::fs::remove_file(output_path);
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
        read_utf8_column(&output_path, 2).last().cloned(),
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

#[test]
fn parser_transform_service_projects_semantic_numeric_columns_into_arrow_types() {
    let output_path = minimal_sas_fixture::unique_tmp_path("transform-semantic-types", "parquet");
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.columns = vec![
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "event_dt".to_string(),
            width: 8,
        },
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "event_date".to_string(),
            width: 8,
        },
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "event_time".to_string(),
            width: 8,
        },
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "elapsed".to_string(),
            width: 8,
        },
    ];
    definition.column_metadata = vec![
        minimal_sas_fixture::FixtureColumnMetadata {
            label: Some("event datetime".to_string()),
            format_name: Some("DATETIME".to_string()),
            informat_name: None,
            format_width: None,
            format_digits: None,
        },
        minimal_sas_fixture::FixtureColumnMetadata {
            label: Some("event date".to_string()),
            format_name: Some("DATE".to_string()),
            informat_name: None,
            format_width: None,
            format_digits: None,
        },
        minimal_sas_fixture::FixtureColumnMetadata {
            label: Some("event time".to_string()),
            format_name: Some("TIME".to_string()),
            informat_name: None,
            format_width: None,
            format_digits: None,
        },
        minimal_sas_fixture::FixtureColumnMetadata {
            label: Some("elapsed duration".to_string()),
            format_name: Some("HOUR".to_string()),
            informat_name: None,
            format_width: None,
            format_digits: None,
        },
    ];
    definition.rows = vec![vec![
        minimal_sas_fixture::FixtureValue::Numeric(315_619_200.0),
        minimal_sas_fixture::FixtureValue::Numeric(3_653.0),
        minimal_sas_fixture::FixtureValue::Numeric(1.5),
        minimal_sas_fixture::FixtureValue::Numeric(2.25),
    ]];

    let loader = InMemorySourceLoader {
        bytes: minimal_sas_fixture::build_fixture(&definition),
    };
    let service = ParserTransformService::new(loader, SupportedSas7bdatParser, LocalParquetSink);
    let report = service
        .run(semantic_fixture_request(output_path.clone()))
        .expect("semantic fixture should transform to parquet");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(
        read_parquet_schema(&output_path),
        vec![
            (
                "event_dt".to_string(),
                "Timestamp(Microsecond, None)".to_string()
            ),
            ("event_dt__sas_missing_tag".to_string(), "Utf8".to_string()),
            ("event_date".to_string(), "Date32".to_string()),
            (
                "event_date__sas_missing_tag".to_string(),
                "Utf8".to_string()
            ),
            ("event_time".to_string(), "Time64(Microsecond)".to_string()),
            (
                "event_time__sas_missing_tag".to_string(),
                "Utf8".to_string()
            ),
            ("elapsed".to_string(), "Duration(Microsecond)".to_string()),
            ("elapsed__sas_missing_tag".to_string(), "Utf8".to_string()),
        ]
    );
    assert_eq!(read_optional_i64_column(&output_path, 0), vec![Some(0)]);
    assert_eq!(read_optional_i32_column(&output_path, 2), vec![Some(0)]);
    assert_eq!(
        read_optional_i64_column(&output_path, 4),
        vec![Some(1_500_000)]
    );
    assert_eq!(
        read_optional_i64_column(&output_path, 6),
        vec![Some(2_250_000)]
    );
    assert_eq!(
        read_field_metadata(&output_path, "event_dt").get("sas.format_name"),
        Some(&"DATETIME".to_string())
    );
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_preserves_real_date_metadata_in_parquet_schema() {
    let output_path = minimal_sas_fixture::unique_tmp_path("transform-real-dates", "parquet");
    let service = ParserTransformService::new(
        FileSystemSourceLoader,
        SupportedSas7bdatParser,
        LocalParquetSink,
    );
    let report = service
        .run(real_dates_request(output_path.clone()))
        .expect("dates sample should transform to parquet");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(
        read_parquet_schema(&output_path),
        vec![
            ("dt".to_string(), "Timestamp(Microsecond, None)".to_string()),
            ("dt__sas_missing_tag".to_string(), "Utf8".to_string()),
            ("dates".to_string(), "Date32".to_string()),
            ("dates__sas_missing_tag".to_string(), "Utf8".to_string()),
            ("times".to_string(), "Time64(Microsecond)".to_string()),
            ("times__sas_missing_tag".to_string(), "Utf8".to_string()),
        ]
    );
    let dt_metadata = read_field_metadata(&output_path, "dt");
    assert_eq!(
        dt_metadata.get("sas.semantic_type"),
        Some(&"datetime".to_string())
    );
    assert_eq!(
        dt_metadata.get("sas.format_name"),
        Some(&"DATETIME".to_string())
    );
    assert_eq!(
        dt_metadata.get("sas.label"),
        Some(&"a very long label for testing accuracy of transformations".to_string())
    );
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn parser_transform_service_preserves_real_special_missing_values_with_sidecar_tags() {
    let output_path = minimal_sas_fixture::unique_tmp_path("transform-real-missings", "parquet");
    let service = ParserTransformService::new(
        FileSystemSourceLoader,
        SupportedSas7bdatParser,
        LocalParquetSink,
    );
    let report = service
        .run(real_missing_request(output_path.clone()))
        .expect("missing_test sample should transform to parquet");

    assert_eq!(report.status, TransformStatus::ParquetWritten);
    assert_eq!(read_optional_float64_column(&output_path, 0), vec![None]);
    assert_eq!(
        read_optional_utf8_column(&output_path, 1),
        vec![Some("A".to_string())]
    );
    assert_eq!(read_optional_float64_column(&output_path, 2), vec![None]);
    assert_eq!(
        read_optional_utf8_column(&output_path, 3),
        vec![Some(".".to_string())]
    );
    assert_eq!(
        read_optional_float64_column(&output_path, 4),
        vec![Some(1.0)]
    );
    assert_eq!(read_optional_utf8_column(&output_path, 5), vec![None]);
    let _ = std::fs::remove_file(output_path);
}

fn example_request(output_path: PathBuf) -> TransformRequest {
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

fn narrow_numeric_request(output_path: PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/deferred-numeric.sas7bdat".into(),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: vec!["measure".to_string()],
            filter: None,
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 8,
                worker_threads: Some(1),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn narrow_numeric_filtered_request(output_path: PathBuf) -> TransformRequest {
    let mut request = narrow_numeric_request(output_path);
    request.transform.filter = Some("measure >= 1".to_string());
    request
}

fn truncated_numeric_bytes(
    layout: minimal_sas_fixture::FixtureLayout,
    value: f64,
    width: usize,
) -> Vec<u8> {
    assert!(
        (1..=8).contains(&width),
        "numeric width must be between 1 and 8 bytes"
    );
    let raw = match layout.endianness {
        Endianness::Little => value.to_le_bytes().to_vec(),
        Endianness::Big => value.to_be_bytes().to_vec(),
    };
    truncate_numeric_storage_bytes(raw, layout.endianness, width)
}

fn truncated_missing_numeric_bytes(
    layout: minimal_sas_fixture::FixtureLayout,
    tag: char,
    width: usize,
) -> Vec<u8> {
    assert!(
        (1..=8).contains(&width),
        "numeric width must be between 1 and 8 bytes"
    );
    let raw = minimal_sas_fixture::tagged_missing_numeric_bytes(layout, tag);
    truncate_numeric_storage_bytes(raw, layout.endianness, width)
}

fn truncate_numeric_storage_bytes(raw: Vec<u8>, endianness: Endianness, width: usize) -> Vec<u8> {
    match endianness {
        Endianness::Little => raw[8 - width..].to_vec(),
        Endianness::Big => raw[..width].to_vec(),
    }
}

fn bounded_memory_request(output_path: PathBuf) -> TransformRequest {
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

fn parallel_batch_request(output_path: PathBuf) -> TransformRequest {
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

fn semantic_fixture_request(output_path: PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: "fixtures/semantic-types.sas7bdat".into(),
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
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn real_dates_request(output_path: PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("sample-sas-datasets")
                .join("dates.sas7bdat"),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: vec!["dt".to_string(), "dates".to_string(), "times".to_string()],
            filter: None,
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 128,
                worker_threads: Some(1),
            },
        },
        sink: SinkContract {
            path: output_path,
            format: SinkFormat::Parquet,
        },
    }
}

fn real_missing_request(output_path: PathBuf) -> TransformRequest {
    TransformRequest {
        source: SourceContract {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("sample-sas-datasets")
                .join("missing_test.sas7bdat"),
            format: SourceFormat::Sas7bdat,
        },
        decoder: DecoderContract {
            mode: DecodeMode::StreamingPages,
        },
        transform: TransformContract {
            selection: vec!["var1".to_string(), "var8".to_string(), "var9".to_string()],
            filter: None,
            execution: ExecutionModel::Streaming,
            tuning: TransformTuning {
                batch_size_rows: 16,
                worker_threads: Some(1),
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

fn read_parquet_schema(path: &Path) -> Vec<(String, String)> {
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

fn read_total_rows(path: &Path) -> usize {
    let reader = File::open(path).expect("parquet output should exist");
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .expect("parquet file should be readable")
        .build()
        .expect("record batch reader should build");
    record_reader
        .map(|batch| batch.expect("record batch should decode").num_rows())
        .sum()
}

fn read_optional_float64_column(path: &Path, column_index: usize) -> Vec<Option<f64>> {
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
                .map(|index| (!array.is_null(index)).then(|| array.value(index)))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_float64_column(path: &Path, column_index: usize) -> Vec<f64> {
    read_optional_float64_column(path, column_index)
        .into_iter()
        .map(|value| value.expect("column should not contain nulls in this assertion"))
        .collect()
}

fn read_optional_utf8_column(path: &Path, column_index: usize) -> Vec<Option<String>> {
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
                .map(|index| (!array.is_null(index)).then(|| array.value(index).to_string()))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_utf8_column(path: &Path, column_index: usize) -> Vec<String> {
    read_optional_utf8_column(path, column_index)
        .into_iter()
        .map(|value| value.expect("column should not contain nulls in this assertion"))
        .collect()
}

fn read_optional_i32_column(path: &Path, column_index: usize) -> Vec<Option<i32>> {
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
                .downcast_ref::<Date32Array>()
                .expect("column should be Date32");
            (0..array.len())
                .map(|index| (!array.is_null(index)).then(|| array.value(index)))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_optional_i64_column(path: &Path, column_index: usize) -> Vec<Option<i64>> {
    let reader = File::open(path).expect("parquet output should exist");
    let record_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .expect("parquet file should be readable")
        .build()
        .expect("record batch reader should build");
    record_reader
        .flat_map(|batch| {
            let batch = batch.expect("record batch should decode");
            if let Some(array) = batch
                .column(column_index)
                .as_any()
                .downcast_ref::<TimestampMicrosecondArray>()
            {
                return (0..array.len())
                    .map(|index| (!array.is_null(index)).then(|| array.value(index)))
                    .collect::<Vec<_>>();
            }
            if let Some(array) = batch
                .column(column_index)
                .as_any()
                .downcast_ref::<Time64MicrosecondArray>()
            {
                return (0..array.len())
                    .map(|index| (!array.is_null(index)).then(|| array.value(index)))
                    .collect::<Vec<_>>();
            }
            let array = batch
                .column(column_index)
                .as_any()
                .downcast_ref::<DurationMicrosecondArray>()
                .expect("column should be Timestamp, Time64, or Duration in microseconds");
            (0..array.len())
                .map(|index| (!array.is_null(index)).then(|| array.value(index)))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_field_metadata(path: &Path, field_name: &str) -> HashMap<String, String> {
    let reader = File::open(path).expect("parquet output should exist");
    let builder =
        ParquetRecordBatchReaderBuilder::try_new(reader).expect("parquet file should be readable");
    builder
        .schema()
        .field_with_name(field_name)
        .expect("field should exist")
        .metadata()
        .clone()
}
