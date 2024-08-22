pub mod builders;
pub mod elements;

use url::ParseError as ParseErrorUrl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyDocument,
    EmptyContent,
    UnexpectedChar(char),
    UnexpectedString(String),
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
            crate::builders::Error::IncompleteData => Self::IncompleteBuilderData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(self) enum ParseToken {
    RepeatSpecial(char, usize),
    String(String),
}

#[inline]
fn process_char(
    c: char,
    last_c: &mut char,
    counter: &mut usize,
    tokens: &mut Vec<ParseToken>,
    src: &mut String,
) {
    if *last_c == '\0' && src.is_empty() {
        *last_c = c;
        return;
    }
    if *last_c == c {
        *counter += 1;
        return;
    } else if src.is_empty() {
        tokens.push(ParseToken::RepeatSpecial(*last_c, *counter + 1))
    } else {
        tokens.push(ParseToken::String(src.clone()));
        src.clear()
    }
    *last_c = c;
    *counter = 0;
}

#[inline]
pub(self) fn tokenize(s: &str) -> Vec<ParseToken> {
    let mut tokens = Vec::new();
    let mut last_c = '\0';
    let mut counter: usize = 0;
    let mut src = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' => {
                if last_c != '\0' {
                    tokens.push(ParseToken::RepeatSpecial(last_c, counter + 1));
                    last_c = '\0';
                }
                src.push(c)
            }
            'a'..='z' => {
                if last_c != '\0' {
                    tokens.push(ParseToken::RepeatSpecial(last_c, counter + 1));
                    last_c = '\0';
                }
                src.push(c)
            }
            ' ' => {
                if src.is_empty() {
                    process_char(c, &mut last_c, &mut counter, &mut tokens, &mut src)
                } else {
                    src.push(c);
                }
            }
            _ => process_char(c, &mut last_c, &mut counter, &mut tokens, &mut src),
        }
    }
    if last_c != '\0' {
        tokens.push(ParseToken::RepeatSpecial(last_c, counter + 1))
    }
    return tokens;
}

mod tests {
    #[test]
    fn tokenize() {
        use super::ParseToken;
        assert_eq!(
            vec![
                ParseToken::RepeatSpecial('*', 2),
                ParseToken::String("bold text".to_string()),
                ParseToken::RepeatSpecial('*', 2)
            ],
            super::tokenize("**bold text**")
        );
        assert_eq!(
            vec![
                ParseToken::RepeatSpecial('!', 1),
                ParseToken::RepeatSpecial('[', 1),
                ParseToken::String("link".to_string()),
                ParseToken::RepeatSpecial(']', 1),
                ParseToken::RepeatSpecial('(', 1),
                ParseToken::String("https".to_string()),
                ParseToken::RepeatSpecial(':', 1),
                ParseToken::RepeatSpecial('/', 2),
                ParseToken::String("example".to_string()),
                ParseToken::RepeatSpecial('.', 1),
                ParseToken::String("com".to_string()),
                ParseToken::RepeatSpecial(')', 1),
            ],
            super::tokenize("![link](https://example.com)")
        );
        assert_eq!(
            vec![
                ParseToken::RepeatSpecial('`', 2),
                ParseToken::String("code".to_string()),
                ParseToken::RepeatSpecial('`', 2)
            ],
            super::tokenize("``code``")
        );
        assert_eq!(
            vec![
                ParseToken::RepeatSpecial('`', 3),
                ParseToken::String("code".to_string()),
                ParseToken::RepeatSpecial('\n', 1),
                ParseToken::String("a type of code".to_string()),
                ParseToken::RepeatSpecial('\n', 1),
                ParseToken::RepeatSpecial('`', 3)
            ],
            super::tokenize("```code\na type of code\n```")
        );
    }
}
