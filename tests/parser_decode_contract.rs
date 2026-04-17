#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::parser::contracts::{ColumnKind, ParsedValue, ParserInput};
use sas_rs::parser::{Sas7bdatParser, SupportedSas7bdatParser};

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
    assert_eq!(parsed.metadata.columns[0].kind, ColumnKind::Numeric64);
    assert_eq!(parsed.metadata.columns[0].offset, 0);
    assert_eq!(parsed.metadata.columns[0].width, 8);
    assert_eq!(parsed.metadata.columns[1].name, "code");
    assert_eq!(parsed.metadata.columns[1].kind, ColumnKind::String);
    assert_eq!(parsed.metadata.columns[1].offset, 8);
    assert_eq!(parsed.metadata.columns[1].width, 4);

    let first_batch = parsed
        .next_batch(2)
        .expect("batch decoding should succeed")
        .expect("expected a first batch");
    assert_eq!(first_batch.row_index_start, 0);
    assert_eq!(first_batch.rows.len(), 2);
    assert_eq!(
        first_batch.rows[0].values,
        vec![
            ParsedValue::Numeric(1.0),
            ParsedValue::String("ABCD".to_string()),
        ]
    );
    assert_eq!(
        first_batch.rows[1].values,
        vec![
            ParsedValue::Numeric(2.5),
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
            ParsedValue::Numeric(3.0),
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
