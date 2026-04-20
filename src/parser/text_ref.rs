#[derive(Debug, Clone, Copy, Default)]
pub struct TextRef {
    pub index: usize,
    pub offset: usize,
    pub length: usize,
}

impl TextRef {
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
