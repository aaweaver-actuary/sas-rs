#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::parser::contracts::{
    CompressionMode, Endianness, ParserInput, UnsupportedFeature, WordSize,
};
use sas_rs::parser::{
    ParserConstants, ParserError, ParserOffsets, Sas7bdatParser, SasFileHeader, SasLayout,
    SasPageKind, SasSubheaderSignature, SupportedSas7bdatParser,
};

#[test]
fn parser_domain_types_classify_headers_pages_and_subheaders() {
    let fixture = minimal_sas_fixture::supported_fixture_bytes();
    let offsets = ParserOffsets::new();
    let header =
        SasFileHeader::from_header_prefix(&fixture[..offsets.header_prefix_len()], &offsets)
            .expect("supported fixture header should classify cleanly");
    let constants = ParserConstants::shared();

    assert_eq!(header.layout, SasLayout::bit64_little());
    assert_eq!(header.text_encoding_code, 20);
    assert_eq!(header.table_name, "DATASET");
    assert_eq!(
        SasPageKind::from_page_type(constants.page_types.meta),
        SasPageKind::Meta
    );
    assert_eq!(
        SasPageKind::from_page_type(constants.page_types.mix),
        SasPageKind::Mix
    );
    assert_eq!(
        SasSubheaderSignature::from_raw(constants.subheader_signatures.row_size),
        SasSubheaderSignature::RowSize
    );
    assert_eq!(
        SasSubheaderSignature::from_raw(constants.subheader_signatures.column_name),
        SasSubheaderSignature::ColumnName
    );
}

#[test]
fn shared_layout_contract_exposes_the_page_and_row_offsets_used_by_parser_and_fixtures() {
    let layout_32 = SasLayout::bit32_little();
    let page_32 = layout_32.page_header_layout();
    let row_32 = layout_32.row_size_layout();
    let format_32 = layout_32.column_format_layout();

    assert_eq!(layout_32.word_size_bytes(), 4);
    assert_eq!(page_32.size, 24);
    assert_eq!(page_32.page_type_offset, 16);
    assert_eq!(page_32.block_count_offset, 18);
    assert_eq!(page_32.subheader_count_offset, 20);
    assert_eq!(page_32.page_subheader_count_offset, 22);
    assert_eq!(row_32.row_length, 20);
    assert_eq!(row_32.row_count, 24);
    assert_eq!(row_32.column_count, 36);
    assert_eq!(row_32.page_row_count, 60);
    assert_eq!(row_32.page_size, 52);
    assert_eq!(format_32.min_len, 46);
    assert_eq!(format_32.format_width_offset, None);
    assert_eq!(format_32.format_digits_offset, None);
    assert_eq!(format_32.format_ref_offset, 34);
    assert_eq!(format_32.label_ref_offset, 40);

    let layout_64 = SasLayout::bit64_big();
    let page_64 = layout_64.page_header_layout();
    let row_64 = layout_64.row_size_layout();
    let format_64 = layout_64.column_format_layout();

    assert_eq!(layout_64.word_size_bytes(), 8);
    assert_eq!(page_64.size, 40);
    assert_eq!(page_64.page_type_offset, 32);
    assert_eq!(page_64.block_count_offset, 34);
    assert_eq!(page_64.subheader_count_offset, 36);
    assert_eq!(page_64.page_subheader_count_offset, 38);
    assert_eq!(row_64.row_length, 40);
    assert_eq!(row_64.row_count, 48);
    assert_eq!(row_64.column_count, 72);
    assert_eq!(row_64.page_row_count, 120);
    assert_eq!(row_64.page_size, 104);
    assert_eq!(format_64.min_len, 58);
    assert_eq!(format_64.format_width_offset, Some(24));
    assert_eq!(format_64.format_digits_offset, Some(26));
    assert_eq!(format_64.format_ref_offset, 46);
    assert_eq!(format_64.label_ref_offset, 52);
}

#[test]
fn parser_constants_group_the_shared_sas_markers_used_by_parser_and_fixture_builders() {
    let constants = ParserConstants::shared();

    assert_eq!(constants.magic_number.len(), 32);
    assert_eq!(constants.layout_flags.bit32, 0x22);
    assert_eq!(constants.layout_flags.bit64, 0x33);
    assert_eq!(constants.endianness_flags.big, 0x00);
    assert_eq!(constants.endianness_flags.little, 0x01);
    assert_eq!(constants.page_types.meta, 0x0000);
    assert_eq!(constants.page_types.data, 0x0100);
    assert_eq!(constants.page_types.mix, 0x0200);
    assert_eq!(constants.subheader_signatures.row_size, 0xF7F7F7F7);
    assert_eq!(constants.subheader_signatures.column_attrs, 0xFFFFFFFC);
    assert_eq!(constants.column_types.numeric, 0x01);
    assert_eq!(constants.column_types.string, 0x02);
    assert_eq!(constants.compression_signatures.row, "SASYZCRL");
    assert_eq!(constants.compression_signatures.binary, "SASYZCR2");
}

#[test]
fn shared_layout_contract_round_trips_header_markers() {
    assert_eq!(
        SasLayout::from_markers(0x22, 0x01),
        Some(SasLayout::bit32_little())
    );
    assert_eq!(
        SasLayout::from_markers(0x33, 0x00),
        Some(SasLayout::bit64_big())
    );
    assert_eq!(SasLayout::from_markers(0x99, 0x01), None);
    assert_eq!(SasLayout::from_markers(0x33, 0x99), None);
}

#[test]
fn supported_subset_is_named_and_exposed_in_the_metadata() {
    let parser = SupportedSas7bdatParser;

    let parsed = parser
        .parse(ParserInput::from_bytes(
            "supported.sas7bdat",
            minimal_sas_fixture::supported_fixture_bytes(),
        ))
        .expect("supported fixture should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-64le-uncompressed-v1");
    assert_eq!(parsed.metadata.subset.word_size, WordSize::Bit64);
    assert_eq!(parsed.metadata.subset.endianness, Endianness::Little);
    assert_eq!(parsed.metadata.subset.compression, CompressionMode::None);
}

#[test]
fn parser_exposes_32_bit_little_endian_layout_metadata() {
    let parser = SupportedSas7bdatParser;

    let parsed = parser
        .parse(ParserInput::from_bytes(
            "32le.sas7bdat",
            minimal_sas_fixture::bit32_little_endian_fixture_bytes(),
        ))
        .expect("32-bit little-endian files should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-32le-uncompressed-v1");
    assert_eq!(parsed.metadata.subset.word_size, WordSize::Bit32);
    assert_eq!(parsed.metadata.subset.endianness, Endianness::Little);
    assert_eq!(parsed.metadata.subset.compression, CompressionMode::None);
}

#[test]
fn parser_exposes_32_bit_layout_metadata_when_header_offsets_are_padded() {
    let parser = SupportedSas7bdatParser;

    let parsed = parser
        .parse(ParserInput::from_bytes(
            "32le-padded.sas7bdat",
            minimal_sas_fixture::bit32_little_endian_padded_header_fixture_bytes(),
        ))
        .expect("32-bit files with padded header offsets should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-32le-uncompressed-v1");
    assert_eq!(parsed.metadata.subset.word_size, WordSize::Bit32);
    assert_eq!(parsed.metadata.subset.endianness, Endianness::Little);
    assert_eq!(parsed.metadata.subset.compression, CompressionMode::None);
}

#[test]
fn parser_exposes_big_endian_layout_metadata() {
    let parser = SupportedSas7bdatParser;

    let parsed = parser
        .parse(ParserInput::from_bytes(
            "big-endian.sas7bdat",
            minimal_sas_fixture::big_endian_fixture_bytes(),
        ))
        .expect("big-endian files should parse");

    assert_eq!(parsed.metadata.subset.name, "sas7bdat-64be-uncompressed-v1");
    assert_eq!(parsed.metadata.subset.word_size, WordSize::Bit64);
    assert_eq!(parsed.metadata.subset.endianness, Endianness::Big);
    assert_eq!(parsed.metadata.subset.compression, CompressionMode::None);
}

#[test]
fn parser_rejects_malformed_word_size_headers() {
    let parser = SupportedSas7bdatParser;

    let error = parser
        .parse(ParserInput::from_bytes(
            "malformed-word-size.sas7bdat",
            minimal_sas_fixture::malformed_word_size_fixture_bytes(0x11),
        ))
        .expect_err("malformed word-size headers should be rejected");

    assert_eq!(
        error,
        ParserError::InvalidFormat("invalid sas7bdat word-size flag")
    );
}

#[test]
fn unsupported_page_types_return_a_structured_error() {
    let parser = SupportedSas7bdatParser;

    let error = parser
        .parse(ParserInput::from_bytes(
            "unsupported-page.sas7bdat",
            minimal_sas_fixture::unsupported_page_type_fixture_bytes(0x0800),
        ))
        .expect_err("unsupported page types should stay explicit");

    assert_eq!(
        error,
        ParserError::Unsupported(UnsupportedFeature::PageType(0x0800))
    );
}
