pub mod block;
pub mod list;
pub mod text;

#[cfg(test)]
mod tests;

use std::slice::Iter;
use url::ParseError as ParseErrorUrl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyDocument,
    EmptyContent,
    UnexpectedChar(char),
    UnexpectedString(String),
    UnexpectedEnd,
    InvalidUrl(ParseErrorUrl),
    IncompleteBuilderData,
}

pub trait Builder: Default {
    type Output;
    fn build(self) -> Result<Self::Output, Error>;
}

pub(crate) trait Element: ToString + Sized {
    fn parse(iter: &mut Iter<ParseToken>) -> Result<Self, ParseError>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncompleteData,
}

pub trait Parser<Out, Src = &'static str> {
    fn parse(src: Src) -> Result<Out, ParseError>;
}

impl From<crate::Error> for ParseError {
    fn from(value: crate::Error) -> Self {
        match value {
            crate::Error::IncompleteData => Self::IncompleteBuilderData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(self) enum ParseToken {
    RepeatSpecial(char, usize),
    String(String),
    Number(usize, Option<usize>),
}

impl ToString for ParseToken {
    fn to_string(&self) -> String {
        match self {
            Self::Number(p, d) => {
                if let Some(f) = d {
                    format!("{}.{}", p, f)
                } else {
                    p.to_string()
                }
            }
            Self::RepeatSpecial(c, n) => c.to_string().repeat(*n),
            Self::String(s) => s.clone(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_str {
    ($type:ident) => {
        impl std::str::FromStr for $type {
            type Err = crate::ParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::Element;
                let tokens = crate::tokenize(s);
                let mut iter = tokens.iter();
                let val = {
                    match Self::parse(&mut iter) {
                        Ok(t) => t,
                        Err(e) => return Err(e),
                    }
                };
                crate::token_expect!(iter);
                return Ok(val);
            }
        }
    };
}

#[macro_export]
macro_rules! token_expect {
    ($iter:ident) => {
        if let Some(t) = $iter.next() {
            return match t {
                crate::ParseToken::String(s) => {
                    Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
                crate::ParseToken::RepeatSpecial(c, _) => {
                    Err(crate::ParseError::UnexpectedChar(*c))
                }
                crate::ParseToken::Number(p, d) => {
                    let s = if let Some(f) = d {
                        format!("{}.{}", p, f)
                    } else {
                        p.to_string()
                    };
                    Err(crate::ParseError::UnexpectedString(s))
                }
            };
        }
    };
    ($iter:ident, $char:literal, $num:literal) => {
        if let Some(t) = $iter.next() {
            match t {
                ParseToken::RepeatSpecial($char, $num) => {}
                ParseToken::RepeatSpecial(c, _) => {
                    return Err(crate::ParseError::UnexpectedChar(*c))
                }
                ParseToken::String(s) => {
                    return Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
                ParseToken::Number(_, _) => {
                    return Err(crate::ParseError::UnexpectedString(t.to_string()))
                }
            }
        } else {
            return Err(crate::ParseError::UnexpectedEnd);
        }
    };
    ($iter:ident, $char:literal) => {
        if let Some(t) = $iter.next() {
            match t {
                ParseToken::RepeatSpecial($char, _) => {}
                ParseToken::RepeatSpecial(c, _) => {
                    return Err(crate::ParseError::UnexpectedChar(*c))
                }
                ParseToken::String(s) => {
                    return Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
            }
        } else {
            return Err(crate::ParseError::UnexpectedEnd);
        }
    };
}

#[macro_export]
macro_rules! token_ignore_char {
    ($iter:ident, $char:literal) => {
        if let Some(t) = $iter.next() {
            crate::token_ignore_char!($iter, $char, t)
        } else {
            return Err(crate::ParseError::UnexpectedEnd);
        }
    };
    ($iter:ident, $char:literal, $tok:ident) => {
        if let crate::ParseToken::RepeatSpecial($char, _) = $tok {
            if let Some(t) = $iter.next() {
                t
            } else {
                return Err(crate::ParseError::UnexpectedEnd);
            }
        } else {
            $tok
        }
    };
    ($iter:ident, $char:literal, $else:expr) => {
        if let Some(t) = $iter.next() {
            crate::token_ignore_char!($iter, $char, t)
        } else {
            $else
        }
    };
    ($iter:ident, $char:literal, $num:literal) => {
        if let Some(t) = $iter.next() {
            crate::token_ignore_char!($iter, $char, $num, t);
        } else {
            return Err(crate::ParseError::UnexpectedEnd);
        }
    };
    ($iter:ident, $char:literal, $num:literal, $tok:ident) => {
        if let ParseToken::RepeatSpecial($char, $num) = $tok {
            if let Some(t) = $iter.next() {
                (t, $num)
            } else {
                return Err(crate::ParseError::UnexpectedEnd);
            }
        } else {
            ($tok, 0)
        }
    };
}

#[macro_export]
macro_rules! token_combine_except {
    ($iter:ident, $($clause:pat, $action:expr),+) => {{
        let mut combined = String::new();
        while let Some(t) = $iter.next() {
            match t {
                $($clause => $action,)+
                crate::ParseToken::String(s) => combined.push_str(s),
                crate::ParseToken::RepeatSpecial(c, n) => {
                    combined.push_str(&c.to_string().repeat(*n))
                }
            }
        }
        combined
    }};
    ($first:ident, $iter:ident, $($clause:pat, $action:expr),+) => {{
        let mut combined = String::new();
        let mut item = Some($first);
        while let Some(t) = item {
            match t {
                $($clause => $action,)+
                crate::ParseToken::String(s) => combined.push_str(s),
                crate::ParseToken::RepeatSpecial(c, n) => {
                    combined.push_str(&c.to_string().repeat(*n))
                },
                crate::ParseToken::Number(_, _) => combined.push_str(&t.to_string()),
            }
            item = $iter.next();
        }
        combined
    }};
}

#[inline]
fn process_char(
    c: char,
    last_c: &mut char,
    counter: &mut usize,
    tokens: &mut Vec<ParseToken>,
    src: &mut String,
    nu: &mut bool,
) {
    if *last_c == '\0' && src.is_empty() {
        *last_c = c;
        return;
    }
    if c >= '0' && c <= '9' {
        if src.is_empty() {
            if *last_c != '\0' {
                tokens.push(ParseToken::RepeatSpecial(*last_c, *counter + 1))
            }
            src.push(c);
            *nu = true
        } else {
            src.push(c)
        }
        *last_c = c;
        *counter = 0;
        return;
    }
    if *last_c == c {
        *counter += 1;
        return;
    } else if src.is_empty() {
        tokens.push(ParseToken::RepeatSpecial(*last_c, *counter + 1))
    } else {
        let t = if *nu {
            let mut s = src.split('.');
            let f = s.nth(0).unwrap().parse().unwrap();
            let k = if let Some(k) = s.nth(1) {
                Some(k.parse().unwrap())
            } else {
                None
            };
            *nu = false;
            ParseToken::Number(f, k)
        } else {
            ParseToken::String(src.clone())
        };
        tokens.push(t);
        src.clear()
    }
    *last_c = c;
    *counter = 0;
}

pub(self) fn tokenize(s: &str) -> Vec<ParseToken> {
    let mut tokens = Vec::new();
    let mut last_c = '\0';
    let mut counter: usize = 0;
    let mut src = String::new();
    let mut number = false;
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
            '1'..='9' | ' ' => {
                if src.is_empty() {
                    process_char(
                        c,
                        &mut last_c,
                        &mut counter,
                        &mut tokens,
                        &mut src,
                        &mut number,
                    )
                } else {
                    src.push(c);
                }
            }
            _ => process_char(
                c,
                &mut last_c,
                &mut counter,
                &mut tokens,
                &mut src,
                &mut number,
            ),
        }
    }
    if last_c != '\0' {
        tokens.push(ParseToken::RepeatSpecial(last_c, counter + 1))
    } else if !src.is_empty() {
        tokens.push(ParseToken::String(src))
    }
    return tokens;
}
