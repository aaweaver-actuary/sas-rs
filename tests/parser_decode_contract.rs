#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::parser::contracts::{
    ColumnKind, ColumnMetadata, NumericValue, ParsedValue, ParserInput, SemanticTypeHint,
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
    assert_eq!(first_batch.row_index_start, 0);
    assert_eq!(first_batch.rows.len(), 2);
    assert_eq!(
        first_batch.rows[0].values,
        vec![
            ParsedValue::Numeric(1.0.into()),
            ParsedValue::String("ABCD".to_string()),
        ]
    );
    assert_eq!(
        first_batch.rows[1].values,
        vec![
            ParsedValue::Numeric(2.5.into()),
            ParsedValue::String("EFGH".to_string()),
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
            ParsedValue::String("IJKL".to_string()),
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

    assert_eq!(
        parsed.metadata.subset.name,
        "sas7bdat-32le-uncompressed-v1"
    );
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
                    ParsedValue::String("ABCD".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(2.5.into()),
                    ParsedValue::String("EFGH".to_string()),
                ],
            },
            sas_rs::parser::contracts::ParsedRow {
                values: vec![
                    ParsedValue::Numeric(3.0.into()),
                    ParsedValue::String("IJKL".to_string()),
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
            ParsedValue::Numeric(NumericValue::deferred_bytes(vec![0x78, 0x56, 0x34, 0x12])),
            ParsedValue::String("ABCD".to_string()),
        ]
    );
}

#[test]
fn parser_reads_the_real_fts0003_file_through_the_existing_entrypoint() {
    match probe_fts0003_via_parser_entrypoint() {
        RealFileProbeOutcome::Readable {
            row_count,
            decoded_rows,
        } => assert_eq!(
            decoded_rows, row_count,
            "if fts0003 becomes readable, the real-file probe should drain the full streamed decode path"
        ),
        RealFileProbeOutcome::Unsupported { stage, detail } => {
            assert_eq!(
                (stage, detail.as_str()),
                ("parse", "InvalidFormat(\"invalid sas7bdat word-size flag\")"),
                "the real-file probe should surface malformed word-size headers before later compatibility checks"
            );
        }
    }
}
