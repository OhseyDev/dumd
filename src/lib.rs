pub mod builders;
pub mod elements;

use url::ParseError as ParseErrorUrl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyDocument,
    EmptyContent,
    UnexpectedChar(char),
    UnexpectedEnd,
    UrlError(ParseErrorUrl),
}

pub trait Parser<Out, Src = &'static str> {
    fn parse(src: Src) -> Result<Out, ParseError>;
}
