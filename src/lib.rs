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
    IncompleteBuilderData,
}

pub trait Parser<Out, Src = &'static str> {
    fn parse(src: Src) -> Result<Out, ParseError>;
}

impl From<crate::builders::Error> for ParseError {
    fn from(value: crate::builders::Error) -> Self {
        match value {
            crate::builders::Error::IncompleteData => Self::IncompleteBuilderData
        }
    }
}
