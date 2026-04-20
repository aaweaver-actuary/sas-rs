#[derive(Debug, Clone, PartialEq)]
pub struct RowBatch {
    pub row_index_start: usize,
    pub rows: Vec<ParsedRow>,
}
