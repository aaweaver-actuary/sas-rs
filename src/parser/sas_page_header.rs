use crate::parser::{constants::SAS7BDAT_MAX_PAGE_HEADER_SIZE, sas_page_type::SasPageType};

use super::{ParserDataSource, ParserError, SasLayout, read_page_header, read_u16};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SasPageHeader {
    pub raw_page_type: u16,
    pub kind: SasPageType,
    pub raw_row_count: usize,
    pub subheader_count: usize,
    pub uses_compressed_storage: bool,
}

impl SasPageHeader {
    pub(crate) fn read(
        reader: &mut dyn ParserDataSource,
        header_size: usize,
        page_size: usize,
        page_index: usize,
        layout: SasLayout,
    ) -> Result<Self, ParserError> {
        let page_header_layout = layout.page_header_layout();
        let mut page_header_buffer = [0_u8; SAS7BDAT_MAX_PAGE_HEADER_SIZE];
        read_page_header(
            reader,
            header_size,
            page_size,
            page_index,
            &mut page_header_buffer[..page_header_layout.size],
        )?;
        let page_header = &page_header_buffer[..page_header_layout.size];
        let raw_page_type = read_u16(
            page_header,
            page_header_layout.page_type_offset,
            layout.endianness,
        )?;

        Ok(Self {
            raw_page_type,
            kind: SasPageType::from_code(raw_page_type),
            raw_row_count: read_u16(
                page_header,
                page_header_layout.block_count_offset,
                layout.endianness,
            )? as usize,
            subheader_count: read_u16(
                page_header,
                page_header_layout.subheader_count_offset,
                layout.endianness,
            )? as usize,
            uses_compressed_storage: (raw_page_type & SAS_PAGE_TYPE_COMP) != 0,
        })
    }
}
