pub struct ParserInput<'a> {
    pub source_name: &'a str,
    pub reader: BoxedParserDataSource,
}

impl<'a> ParserInput<'a> {
    pub fn new(source_name: &'a str, reader: BoxedParserDataSource) -> Self {
        Self {
            source_name,
            reader,
        }
    }

    pub fn from_bytes(source_name: &'a str, bytes: Vec<u8>) -> Self {
        Self::from_reader(source_name, Cursor::new(bytes))
    }

    pub fn from_reader<R>(source_name: &'a str, reader: R) -> Self
    where
        R: ParserDataSource + 'static,
    {
        Self::new(source_name, Box::new(reader))
    }
}
