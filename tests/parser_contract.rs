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
fn unsupported_endianness_returns_a_structured_error() {
    let parser = SupportedSas7bdatParser;

    let error = parser
        .parse(ParserInput::from_bytes(
            "big-endian.sas7bdat",
            minimal_sas_fixture::big_endian_fixture_bytes(),
        ))
        .expect_err("big-endian files should be rejected");

    assert_eq!(
        error,
        ParserError::Unsupported(UnsupportedFeature::Endianness(Endianness::Big))
    );
}

#[test]
fn unsupported_compression_returns_a_structured_error() {
    let parser = SupportedSas7bdatParser;

    let error = parser
        .parse(ParserInput::from_bytes(
            "compressed.sas7bdat",
            minimal_sas_fixture::compressed_fixture_bytes(),
        ))
        .expect_err("row-compressed files should be rejected");

    assert_eq!(
        error,
        ParserError::Unsupported(UnsupportedFeature::Compression(CompressionMode::Row))
    );
}
