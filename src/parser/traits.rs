
pub trait Sas7bdatParser {
    fn parse(&self, input: ParserInput<'_>) -> Result<ParsedSas7bdat, ParserError>;
}

pub trait ParserDataSource: Read + Seek + Send {}

impl<T> ParserDataSource for T where T: Read + Seek + Send {}