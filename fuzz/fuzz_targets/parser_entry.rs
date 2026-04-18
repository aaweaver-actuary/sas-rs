#![no_main]

use libfuzzer_sys::fuzz_target;
use sas_rs::parser::{ParserInput, Sas7bdatParser, SupportedSas7bdatParser};

fuzz_target!(|data: &[u8]| {
    let parser = SupportedSas7bdatParser;
    if let Ok(mut parsed) = parser.parse(ParserInput::from_bytes("fuzz.sas7bdat", data.to_vec())) {
        let _ = parsed.next_batch(256);
    }
});
