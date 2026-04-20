use super::contracts::{
    ColumnKind, CompressionMode, Endianness, NumericValue, ParsedRow, ParsedValue, SasMetadata,
    SasMissingTag,
};
use super::{ParserError, UnsupportedFeature, decode_text_bytes};

pub(crate) fn parse_subheader_row(
    page: &[u8],
    offset: usize,
    len: usize,
    compression: CompressionMode,
    row_length: usize,
) -> Result<Vec<u8>, ParserError> {
    let end = offset
        .checked_add(len)
        .ok_or(ParserError::InvalidFormat("subheader row range overflowed"))?;
    let payload = page.get(offset..end).ok_or(ParserError::InvalidFormat(
        "subheader row range is outside the page",
    ))?;
    match compression {
        CompressionMode::None => {
            if payload.len() != row_length {
                return Err(ParserError::InvalidFormat(
                    "row subheader length does not match the declared row length",
                ));
            }
            Ok(payload.to_vec())
        }
        CompressionMode::Row => decompress_row_rle(payload, row_length),
        CompressionMode::Binary => decompress_row_binary(payload, row_length),
        CompressionMode::Unknown(code) => Err(ParserError::Unsupported(
            UnsupportedFeature::Compression(CompressionMode::Unknown(code)),
        )),
    }
}

pub(crate) fn parse_row(
    row: &[u8],
    metadata: &SasMetadata,
    text_encoding_code: u8,
) -> Result<ParsedRow, ParserError> {
    let mut values = Vec::with_capacity(metadata.columns.len());

    for column in &metadata.columns {
        let end = column
            .offset
            .checked_add(column.width)
            .ok_or(ParserError::InvalidFormat("column range overflowed"))?;
        let raw_value = row
            .get(column.offset..end)
            .ok_or(ParserError::InvalidFormat("column value is truncated"))?;

        let value = match column.kind {
            ColumnKind::Numeric => parse_numeric_value(raw_value, metadata.subset.endianness)?,
            ColumnKind::String => {
                ParsedValue::String(decode_text_bytes(raw_value, text_encoding_code)?)
            }
        };
        values.push(value);
    }

    Ok(ParsedRow { values })
}

fn parse_numeric_value(
    raw_value: &[u8],
    endianness: Endianness,
) -> Result<ParsedValue, ParserError> {
    match raw_value.len() {
        0 => Err(ParserError::InvalidFormat(
            "numeric value width must be greater than zero",
        )),
        1..=7 => Ok(ParsedValue::Numeric(NumericValue::deferred_bytes(
            raw_value.to_vec(),
        ))),
        8 => {
            let raw_bits = match endianness {
                Endianness::Little => u64::from_le_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
                Endianness::Big => u64::from_be_bytes(raw_value.try_into().map_err(|_| {
                    ParserError::InvalidFormat("numeric value width must be 8 bytes")
                })?),
            };
            let value = f64::from_bits(raw_bits);
            Ok(ParsedValue::Numeric(NumericValue::Float64 {
                value,
                raw_bits,
                missing_tag: decode_sas_missing_tag(value, raw_bits),
            }))
        }
        width => Err(ParserError::Unsupported(UnsupportedFeature::NumericWidth(
            width as u32,
        ))),
    }
}

fn decode_sas_missing_tag(value: f64, raw_bits: u64) -> Option<SasMissingTag> {
    if !value.is_nan() {
        return None;
    }

    let tag = !((raw_bits >> 40) & 0xFF) as u8;
    match tag {
        0 => Some(SasMissingTag::Underscore),
        2..=27 => Some(SasMissingTag::Letter((b'A' + (tag - 2)) as char)),
        b'_' => Some(SasMissingTag::Underscore),
        b'A'..=b'Z' => Some(SasMissingTag::Letter(tag as char)),
        _ => Some(SasMissingTag::Dot),
    }
}

fn decompress_row_rle(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {
    const COMMAND_LENGTHS: [usize; 16] = [1, 1, 0, 0, 2, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0];
    let mut input_offset = 0;
    let mut output = Vec::with_capacity(row_length);

    while input_offset < payload.len() {
        let control = payload[input_offset];
        input_offset += 1;
        let command = (control & 0xF0) >> 4;
        let length = (control & 0x0F) as usize;
        let command_len = COMMAND_LENGTHS[command as usize];
        if input_offset + command_len > payload.len() {
            return Err(ParserError::InvalidFormat(
                "row-compressed payload is truncated",
            ));
        }

        let mut copy_len = 0_usize;
        let mut insert_len = 0_usize;
        let mut insert_byte = 0_u8;

        match command {
            0 => {
                copy_len = payload[input_offset] as usize + 64 + length * 256;
                input_offset += 1;
            }
            1 => {
                copy_len = payload[input_offset] as usize + 64 + length * 256 + 4096;
                input_offset += 1;
            }
            2 => copy_len = length + 96,
            4 => {
                insert_len = payload[input_offset] as usize + 18 + length * 256;
                insert_byte = payload[input_offset + 1];
                input_offset += 2;
            }
            5 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = b'@';
                input_offset += 1;
            }
            6 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = b' ';
                input_offset += 1;
            }
            7 => {
                insert_len = payload[input_offset] as usize + 17 + length * 256;
                insert_byte = 0;
                input_offset += 1;
            }
            8 => copy_len = length + 1,
            9 => copy_len = length + 17,
            10 => copy_len = length + 33,
            11 => copy_len = length + 49,
            12 => {
                insert_len = length + 3;
                insert_byte = payload[input_offset];
                input_offset += 1;
            }
            13 => {
                insert_len = length + 2;
                insert_byte = b'@';
            }
            14 => {
                insert_len = length + 2;
                insert_byte = b' ';
            }
            15 => {
                insert_len = length + 2;
                insert_byte = 0;
            }
            _ => unreachable!(),
        }

        if copy_len != 0 {
            if output.len() + copy_len > row_length {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload exceeds the declared row length",
                ));
            }
            if input_offset + copy_len > payload.len() {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload is truncated",
                ));
            }
            output.extend_from_slice(&payload[input_offset..input_offset + copy_len]);
            input_offset += copy_len;
        }

        if insert_len != 0 {
            if output.len() + insert_len > row_length {
                return Err(ParserError::InvalidFormat(
                    "row-compressed payload exceeds the declared row length",
                ));
            }
            output.extend(std::iter::repeat_n(insert_byte, insert_len));
        }
    }

    if output.len() != row_length {
        return Err(ParserError::InvalidFormat(
            "row-compressed payload did not decompress to the declared row length",
        ));
    }

    Ok(output)
}

fn decompress_row_binary(payload: &[u8], row_length: usize) -> Result<Vec<u8>, ParserError> {
    let mut input_offset = 0;
    let mut output = Vec::with_capacity(row_length);

    while input_offset + 2 <= payload.len() {
        let prefix = u16::from_be_bytes([payload[input_offset], payload[input_offset + 1]]);
        input_offset += 2;

        for bit_index in 0..16 {
            if output.len() == row_length {
                break;
            }

            let is_control = (prefix & (1 << (15 - bit_index))) != 0;
            if !is_control {
                if input_offset >= payload.len() {
                    break;
                }
                if output.len() + 1 > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                output.push(payload[input_offset]);
                input_offset += 1;
                continue;
            }

            if input_offset + 2 > payload.len() {
                return Err(ParserError::InvalidFormat(
                    "binary-compressed payload is truncated",
                ));
            }

            let marker = payload[input_offset];
            let next = payload[input_offset + 1];
            input_offset += 2;
            let mut insert_len = 0_usize;
            let mut copy_len = 0_usize;
            let mut insert_byte = 0_u8;
            let mut back_offset = 0_usize;

            if marker <= 0x0F {
                insert_len = 3 + marker as usize;
                insert_byte = next;
            } else if (marker >> 4) == 1 {
                if input_offset >= payload.len() {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload is truncated",
                    ));
                }
                insert_len = 19 + (marker & 0x0F) as usize + next as usize * 16;
                insert_byte = payload[input_offset];
                input_offset += 1;
            } else if (marker >> 4) == 2 {
                if input_offset >= payload.len() {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload is truncated",
                    ));
                }
                copy_len = 16 + payload[input_offset] as usize;
                input_offset += 1;
                back_offset = 3 + (marker & 0x0F) as usize + next as usize * 16;
            } else {
                copy_len = (marker >> 4) as usize;
                back_offset = 3 + (marker & 0x0F) as usize + next as usize * 16;
            }

            if insert_len != 0 {
                if output.len() + insert_len > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                output.extend(std::iter::repeat_n(insert_byte, insert_len));
            } else if copy_len != 0 {
                if output.len() < back_offset || copy_len > back_offset {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload contains an invalid back-reference",
                    ));
                }
                if output.len() + copy_len > row_length {
                    return Err(ParserError::InvalidFormat(
                        "binary-compressed payload exceeds the declared row length",
                    ));
                }
                let start = output.len() - back_offset;
                for index in 0..copy_len {
                    let byte = output[start + index];
                    output.push(byte);
                }
            }
        }
    }

    if output.len() != row_length {
        return Err(ParserError::InvalidFormat(
            "binary-compressed payload did not decompress to the declared row length",
        ));
    }

    Ok(output)
}
