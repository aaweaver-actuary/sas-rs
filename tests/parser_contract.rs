#[path = "support/minimal_sas_fixture.rs"]
mod minimal_sas_fixture;

use sas_rs::parser::contracts::{
    CompressionMode, Endianness, ParserInput, UnsupportedFeature, WordSize,
};
use sas_rs::parser::{ParserError, Sas7bdatParser, SupportedSas7bdatParser};

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
