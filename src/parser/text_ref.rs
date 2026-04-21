/// Reference into a metadata text blob.
#[derive(Debug, Clone, Copy, Default)]
pub struct TextRef {
    /// Text-blob index.
    pub index: usize,
    /// Byte offset within that blob.
    pub offset: usize,
    /// Referenced byte length.
    pub length: usize,
}

impl TextRef {
    /// Construct a text reference from blob index, offset, and length.
    pub fn new(index: usize, offset: usize, length: usize) -> Self {
        Self {
            index,
            offset,
            length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_ref_new() {
        let text_ref = TextRef::new(1, 10, 5);
        assert_eq!(text_ref.index, 1);
        assert_eq!(text_ref.offset, 10);
        assert_eq!(text_ref.length, 5);
    }
}
