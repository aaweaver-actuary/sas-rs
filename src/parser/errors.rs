use std::error::Error;

impl Display for ParserError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(message) => formatter.write_str(message),
            Self::Unsupported(feature) => feature.fmt(formatter),
            Self::Io(message) => formatter.write_str(message),
        }
    }
}

impl Error for ParserError {}

