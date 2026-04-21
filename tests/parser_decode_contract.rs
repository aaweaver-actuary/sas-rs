#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::parser::contracts::{
    ColumnKind, ColumnMetadata, NumericValue, ParsedValue, ParserInput, RowValueKind,
    SasMissingTag, SemanticTypeHint,
};
use sas_rs::parser::{Sas7bdatParser, SupportedSas7bdatParser};
use std::fs::File;
use std::path::PathBuf;

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

fn sample_dataset_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sample-sas-datasets")
        .join(file_name)
}

fn probe_sample_via_parser_entrypoint(file_name: &str) -> RealFileProbeOutcome {
    let path = sample_dataset_path(file_name);
    let path_display = path.display().to_string();
    let file = File::open(&path).expect("sample dataset fixture should be readable from disk");
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
        match parsed.next_batch(1_024) {
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

fn probe_fts0003_via_parser_entrypoint() -> RealFileProbeOutcome {
    probe_sample_via_parser_entrypoint("fts0003.sas7bdat")
}

fn assert_real_file_is_readable(file_name: &str) {
    match probe_sample_via_parser_entrypoint(file_name) {
        RealFileProbeOutcome::Readable {
            row_count,
            decoded_rows,
        } => assert_eq!(
            decoded_rows, row_count,
            "{file_name} should drain the full streamed decode path once parsed"
        ),
        RealFileProbeOutcome::Unsupported { stage, detail } => {
            panic!("{file_name} should be readable; failed during {stage}: {detail}");
        }
    }
}

fn drain_all_rows(
    parsed: &mut sas_rs::parser::ParsedSas7bdat,
    batch_size_rows: usize,
) -> Vec<sas_rs::parser::contracts::ParsedRow> {
    let mut rows = Vec::new();
    while let Some(batch) = parsed
        .next_batch(batch_size_rows)
        .expect("batch decoding should succeed")
    {
        rows.extend(batch.rows);
    }
    rows
}

fn numeric_missing_tag(value: &ParsedValue) -> Option<SasMissingTag> {
    match value {
        ParsedValue::Numeric(numeric)
        | ParsedValue::Date(numeric)
        | ParsedValue::Time(numeric)
        | ParsedValue::DateTime(numeric)
        | ParsedValue::Duration(numeric) => numeric.missing_tag(),
        ParsedValue::Character(_) => None,
    }
}

#[test]
fn parser_decodes_metadata_and_batches_from_the_supported_subset_fixture() {
    let parser = SupportedSas7bdatParser;

    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "supported.sas7bdat",
            minimal_sas_fixture::supported_fixture_bytes(),
        ))
        .expect("supported fixture should parse");

    assert_eq!(parsed.metadata.table_name, "DATASET");
    assert_eq!(parsed.metadata.row_count, 3);
    assert_eq!(parsed.metadata.row_length, 12);
    assert_eq!(parsed.metadata.page_size, 4096);
    assert_eq!(parsed.metadata.page_count, 2);
    assert_eq!(parsed.metadata.columns.len(), 2);
    assert_eq!(parsed.metadata.columns[0].name, "customer_id");
    assert_eq!(parsed.metadata.columns[0].kind, ColumnKind::Numeric);
    assert_eq!(parsed.metadata.columns[0].offset, 0);
    assert_eq!(parsed.metadata.columns[0].width, 8);
    assert_eq!(
        parsed.metadata.columns[0].semantic_type,
        SemanticTypeHint::Deferred
    );
    assert_eq!(
        parsed.metadata.columns[0].metadata,
        ColumnMetadata::default()
    );
    assert_eq!(parsed.metadata.columns[1].name, "code");
    assert_eq!(parsed.metadata.columns[1].kind, ColumnKind::String);
    assert_eq!(parsed.metadata.columns[1].offset, 8);
    assert_eq!(parsed.metadata.columns[1].width, 4);
    assert_eq!(
        parsed.metadata.columns[1].semantic_type,
        SemanticTypeHint::Deferred
    );
    assert_eq!(
        parsed.metadata.columns[1].metadata,
        ColumnMetadata::default()
    );

    let first_batch = parsed
        .next_batch(2)
        .expect("batch decoding should succeed")
        .expect("expected a first batch");
    assert_eq!(first_batch.schema.table_name, "DATASET");
    assert_eq!(first_batch.schema.file_label, "");
    assert_eq!(first_batch.schema.columns.len(), 2);
    assert_eq!(first_batch.schema.columns[0].source_index, 0);
    assert_eq!(first_batch.schema.columns[0].name, "customer_id");
    assert_eq!(
        first_batch.schema.columns[0].value_kind,
        RowValueKind::Numeric
    );
    assert_eq!(
        first_batch.schema.columns[0]
            .missing_tag_column_name
            .as_deref(),
        Some("customer_id__sas_missing_tag")
    );
    assert_eq!(first_batch.schema.columns[1].source_index, 1);
    assert_eq!(first_batch.schema.columns[1].name, "code");
    assert_eq!(
        first_batch.schema.columns[1].value_kind,
        RowValueKind::Character
    );
    assert_eq!(first_batch.schema.columns[1].missing_tag_column_name, None);
    assert_eq!(first_batch.row_index_start, 0);
    assert_eq!(first_batch.rows.len(), 2);
    assert_eq!(
        first_batch.rows[0].values,
        vec![
            ParsedValue::Numeric(1.0.into()),
            ParsedValue::Character("ABCD".to_string()),
        ]
    );
    assert_eq!(
        first_batch.rows[1].values,
        vec![
            ParsedValue::Numeric(2.5.into()),
            ParsedValue::Character("EFGH".to_string()),
        ]
    );

    let second_batch = parsed
        .next_batch(2)
        .expect("batch decoding should succeed")
        .expect("expected a second batch");
    assert_eq!(second_batch.row_index_start, 2);
    assert_eq!(second_batch.rows.len(), 1);
    assert_eq!(
        second_batch.rows[0].values,
        vec![
            ParsedValue::Numeric(3.0.into()),
            ParsedValue::Character("IJKL".to_string()),
        ]
    );

    assert!(
        parsed
            .next_batch(2)
            .expect("batch decoding should succeed")
            .is_none(),
        "expected the stream to be exhausted"
    );
}

#[test]
fn parser_decodes_uncompressed_32_bit_little_endian_fixture_end_to_end() {
    let parser = SupportedSas7bdatParser;

    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "32le.sas7bdat",
            minimal_sas_fixture::bit32_little_endian_fixture_bytes(),
        ))
        .expect("32-bit little-endian fixture should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-32le-uncompressed-v1");
    assert_eq!(
        parsed.metadata.subset.word_size,
        sas_rs::parser::contracts::WordSize::Bit32
    );
    assert_eq!(
        parsed
            .next_batch(3)
            .expect("batch decoding should succeed")
            .expect("expected one batch")
            .rows
            .len(),
        3
    );
}

#[test]
fn parser_decodes_latin1_strings_without_claiming_utf8_only_support() {
    let parser = SupportedSas7bdatParser;

    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "latin1.sas7bdat",
            minimal_sas_fixture::latin1_fixture_bytes(),
        ))
        .expect("latin-1 fixture should parse");

    let batch = parsed
        .next_batch(1)
        .expect("batch decoding should succeed")
        .expect("expected one row");

    assert_eq!(
        batch.rows[0].values,
        vec![
            ParsedValue::Numeric(1.0.into()),
            ParsedValue::Character("Caf\u{00E9}".to_string()),
        ]
    );
}

#[test]
fn parser_decodes_uncompressed_big_endian_fixture_end_to_end() {
    let parser = SupportedSas7bdatParser;

    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "big-endian.sas7bdat",
            minimal_sas_fixture::big_endian_fixture_bytes(),
        ))
        .expect("big-endian fixture should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-64be-uncompressed-v1");
    assert_eq!(
        parsed
            .next_batch(3)
            .expect("batch decoding should succeed")
            .expect("expected one batch")
            .rows,
        vec![
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(1.0.into()),
                    ParsedValue::Character("ABCD".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(2.5.into()),
                    ParsedValue::Character("EFGH".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(3.0.into()),
                    ParsedValue::Character("IJKL".to_string()),
                ],
            },
        ]
    );
}

#[test]
fn parser_decodes_supported_subset_across_multiple_data_pages() {
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.rows = (0..700)
        .map(|index| {
            vec![
                minimal_sas_fixture::FixtureValue::Numeric(index as f64),
                minimal_sas_fixture::FixtureValue::String(format!("{index:04}")),
            ]
        })
        .collect();

    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "multi-page.sas7bdat",
            minimal_sas_fixture::build_fixture(&definition),
        ))
        .expect("multi-page fixture should parse");

    assert_eq!(parsed.metadata.row_count, 700);

    let mut seen_rows = 0;
    while let Some(batch) = parsed
        .next_batch(128)
        .expect("batch decoding should succeed")
    {
        seen_rows += batch.rows.len();
    }

    assert_eq!(seen_rows, 700);
}

#[test]
fn parser_defers_multi_page_row_reads_until_batches_are_requested() {
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
    let parse_budget = minimal_sas_fixture::lazy_parse_read_budget(page_count);
    let first_batch_budget = minimal_sas_fixture::first_batch_read_budget(page_count);
    let (reader, monitor) =
        minimal_sas_fixture::tracked_reader(minimal_sas_fixture::build_fixture(&definition));
    let parser = SupportedSas7bdatParser;

    let mut parsed = parser
        .parse(ParserInput::from_reader("streaming.sas7bdat", reader))
        .expect("streaming fixture should parse");

    assert!(
        monitor.bytes_read() <= parse_budget,
        "expected parse to stay within {parse_budget} bytes before batching, read {}",
        monitor.bytes_read()
    );

    let first_batch = parsed
        .next_batch(64)
        .expect("batch decoding should succeed")
        .expect("expected a first batch");

    assert_eq!(first_batch.rows.len(), 64);
    assert!(
        monitor.bytes_read() <= first_batch_budget,
        "expected the first batch to avoid whole-dataset materialization; budget {first_batch_budget}, read {}",
        monitor.bytes_read()
    );
}

#[test]
fn parser_decodes_row_compressed_rows_stored_across_meta_and_mix_pages() {
    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "row-compressed.sas7bdat",
            minimal_sas_fixture::row_compressed_mixed_page_fixture_bytes(),
        ))
        .expect("row-compressed fixture should parse");

    assert_eq!(
        parsed.metadata.subset.compression,
        sas_rs::parser::CompressionMode::Row
    );
    assert_eq!(
        drain_all_rows(&mut parsed, 8),
        vec![
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(1.0.into()),
                    ParsedValue::Character("ABCD".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(2.5.into()),
                    ParsedValue::Character("EFGH".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(3.0.into()),
                    ParsedValue::Character("IJKL".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(4.25.into()),
                    ParsedValue::Character("MNOP".to_string()),
                ],
            },
        ]
    );
}

#[test]
fn parser_decodes_binary_compressed_rows_from_meta_subheaders() {
    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "binary-compressed.sas7bdat",
            minimal_sas_fixture::binary_compressed_fixture_bytes(),
        ))
        .expect("binary-compressed fixture should parse");

    assert_eq!(
        parsed.metadata.subset.compression,
        sas_rs::parser::CompressionMode::Binary
    );
    assert_eq!(
        drain_all_rows(&mut parsed, 8),
        vec![
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(1.0.into()),
                    ParsedValue::Character("ABCD".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(2.5.into()),
                    ParsedValue::Character("EFGH".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(3.0.into()),
                    ParsedValue::Character("IJKL".to_string()),
                ],
            },
        ]
    );
}

#[test]
fn parser_preserves_non_8_byte_numeric_cells_without_parser_core_rejection() {
    let mut definition = minimal_sas_fixture::supported_fixture_definition();
    definition.columns = vec![
        minimal_sas_fixture::FixtureColumn::Numeric {
            name: "narrow_numeric".to_string(),
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

    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "narrow-numeric.sas7bdat",
            minimal_sas_fixture::build_fixture(&definition),
        ))
        .expect("supported uncompressed fixture should parse with a narrow numeric column");

    assert_eq!(parsed.metadata.columns[0].kind, ColumnKind::Numeric);
    assert_eq!(parsed.metadata.columns[0].width, 4);
    assert_eq!(
        parsed.metadata.columns[0].semantic_type,
        SemanticTypeHint::Deferred
    );

    let batch = parsed
        .next_batch(1)
        .expect("batch decoding should succeed")
        .expect("expected one row");
    assert_eq!(
        batch.rows[0].values,
        vec![
            ParsedValue::Numeric(NumericValue::Float64 {
                value: f64::from_bits(0x1234_5678_0000_0000),
                raw_bits: 0x1234_5678_0000_0000,
                missing_tag: None,
            }),
            ParsedValue::Character("ABCD".to_string()),
        ]
    );
}

#[test]
fn parser_reads_the_real_10rec_file_through_the_existing_entrypoint() {
    match probe_sample_via_parser_entrypoint("10rec.sas7bdat") {
        RealFileProbeOutcome::Readable {
            row_count,
            decoded_rows,
        } => assert_eq!(
            decoded_rows, row_count,
            "10rec should drain the full streamed decode path once parsed"
        ),
        RealFileProbeOutcome::Unsupported { stage, detail } => {
            panic!("10rec should be readable within PR-02; failed during {stage}: {detail}");
        }
    }
}

#[test]
fn parser_reads_the_real_fts0003_file_through_the_compressed_entrypoint() {
    match probe_fts0003_via_parser_entrypoint() {
        RealFileProbeOutcome::Readable {
            row_count,
            decoded_rows,
        } => assert_eq!(
            decoded_rows, row_count,
            "fts0003 should now drain the full streamed decode path through compression and mix-page handling"
        ),
        RealFileProbeOutcome::Unsupported { stage, detail } => {
            panic!("fts0003 should be readable within PR-03; failed during {stage}: {detail}");
        }
    }
}

#[test]
fn parser_reads_real_binary_compressed_samples_through_the_existing_entrypoint() {
    for file_name in [
        "dates_binary.sas7bdat",
        "dates_char.sas7bdat",
        "dates_longname_binary.sas7bdat",
        "dates_longname_char.sas7bdat",
        "sample_bincompressed.sas7bdat",
        "test11.sas7bdat",
        "test12.sas7bdat",
        "test14.sas7bdat",
        "test15.sas7bdat",
        "test2.sas7bdat",
        "test3.sas7bdat",
        "test5.sas7bdat",
        "test6.sas7bdat",
        "test8.sas7bdat",
        "test9.sas7bdat",
    ] {
        assert_real_file_is_readable(file_name);
    }
}

#[test]
fn parser_reads_real_non_utf8_samples_through_the_existing_entrypoint() {
    for file_name in [
        "0x40controlbyte.sas7bdat",
        "datetime.sas7bdat",
        "issue1_pandas.sas7bdat",
        "issue_pandas.sas7bdat",
        "max_sas_date.sas7bdat",
        "productsales.sas7bdat",
        "q_del_pandas.sas7bdat",
        "q_pandas.sas7bdat",
        "weigth2.sas7bdat",
        "zero_rows.sas7bdat",
    ] {
        assert_real_file_is_readable(file_name);
    }
}

#[test]
fn parser_decodes_real_gb18030_text_values_honestly() {
    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_reader(
            "issue_pandas.sas7bdat",
            File::open(sample_dataset_path("issue_pandas.sas7bdat"))
                .expect("issue_pandas fixture should open"),
        ))
        .expect("issue_pandas fixture should parse");

    let batch = parsed
        .next_batch(1)
        .expect("batch decoding should succeed")
        .expect("expected one row");

    assert_eq!(
        batch.rows[0].values,
        vec![ParsedValue::Character("小类_长筒袜　".to_string())]
    );
}

#[test]
fn parser_infers_semantic_types_and_column_metadata_from_fixture_formats() {
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
        minimal_sas_fixture::FixtureValue::Numeric(0.0),
        minimal_sas_fixture::FixtureValue::Numeric(0.0),
        minimal_sas_fixture::FixtureValue::Numeric(0.0),
        minimal_sas_fixture::FixtureValue::Numeric(60.0),
    ]];

    let parser = SupportedSas7bdatParser;
    let parsed = parser
        .parse(ParserInput::from_bytes(
            "semantic-fixture.sas7bdat",
            minimal_sas_fixture::build_fixture(&definition),
        ))
        .expect("semantic fixture should parse");

    assert_eq!(
        parsed.metadata.columns[0].semantic_type,
        SemanticTypeHint::DateTime
    );
    assert_eq!(
        parsed.metadata.columns[0].metadata.label.as_deref(),
        Some("event datetime")
    );
    assert_eq!(
        parsed.metadata.columns[0].metadata.format_name.as_deref(),
        Some("DATETIME")
    );
    assert_eq!(
        parsed.metadata.columns[1].semantic_type,
        SemanticTypeHint::Date
    );
    assert_eq!(
        parsed.metadata.columns[2].semantic_type,
        SemanticTypeHint::Time
    );
    assert_eq!(
        parsed.metadata.columns[3].semantic_type,
        SemanticTypeHint::Duration
    );

    let mut parsed = parser
        .parse(ParserInput::from_bytes(
            "semantic-fixture.sas7bdat",
            minimal_sas_fixture::build_fixture(&definition),
        ))
        .expect("semantic fixture should parse for row decoding");
    let batch = parsed
        .next_batch(1)
        .expect("batch decoding should succeed")
        .expect("expected one semantic row");

    assert_eq!(
        batch.rows[0].values,
        vec![
            ParsedValue::DateTime(0.0.into()),
            ParsedValue::Date(0.0.into()),
            ParsedValue::Time(0.0.into()),
            ParsedValue::Duration(60.0.into()),
        ]
    );
}

#[test]
fn parser_preserves_real_dates_fixture_semantic_metadata() {
    let parser = SupportedSas7bdatParser;
    let parsed = parser
        .parse(ParserInput::from_reader(
            "dates.sas7bdat",
            File::open(sample_dataset_path("dates.sas7bdat")).expect("dates fixture should open"),
        ))
        .expect("dates fixture should parse");

    let dt = &parsed.metadata.columns[0];
    assert_eq!(dt.name, "dt");
    assert_eq!(dt.semantic_type, SemanticTypeHint::DateTime);
    assert_eq!(dt.metadata.format_name.as_deref(), Some("DATETIME"));
    assert_eq!(
        dt.metadata.label.as_deref(),
        Some("a very long label for testing accuracy of transformations")
    );

    let dates = &parsed.metadata.columns[3];
    assert_eq!(dates.name, "dates");
    assert_eq!(dates.semantic_type, SemanticTypeHint::Date);
    assert_eq!(dates.metadata.format_name.as_deref(), Some("DATE"));

    let times = &parsed.metadata.columns[5];
    assert_eq!(times.name, "times");
    assert_eq!(times.semantic_type, SemanticTypeHint::Time);
    assert_eq!(times.metadata.format_name.as_deref(), Some("TIME"));
}

#[test]
fn parser_exposes_real_special_missing_tags_without_flattening_them() {
    let parser = SupportedSas7bdatParser;
    let mut parsed = parser
        .parse(ParserInput::from_reader(
            "missing_test.sas7bdat",
            File::open(sample_dataset_path("missing_test.sas7bdat"))
                .expect("missing_test fixture should open"),
        ))
        .expect("missing_test fixture should parse");

    let batch = parsed
        .next_batch(8)
        .expect("batch decoding should succeed")
        .expect("expected one batch");
    let row = &batch.rows[0];

    assert_eq!(
        numeric_missing_tag(&row.values[0]),
        Some(SasMissingTag::Letter('A'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[1]),
        Some(SasMissingTag::Letter('B'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[2]),
        Some(SasMissingTag::Letter('C'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[3]),
        Some(SasMissingTag::Letter('X'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[4]),
        Some(SasMissingTag::Letter('Y'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[5]),
        Some(SasMissingTag::Letter('Z'))
    );
    assert_eq!(
        numeric_missing_tag(&row.values[6]),
        Some(SasMissingTag::Underscore)
    );
    assert_eq!(
        numeric_missing_tag(&row.values[7]),
        Some(SasMissingTag::Dot)
    );
    assert_eq!(numeric_missing_tag(&row.values[8]), None);
}
