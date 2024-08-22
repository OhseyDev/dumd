use std::str::FromStr;

use crate::ParseToken;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeKind {
    Cpp,
    CStandard,
    CSharp,
    Rust,
    Java,
    JavaScript,
    Unknown(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub content: Box<str>,
    pub multiline: Option<CodeKind>,
}

impl FromStr for Code {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = crate::tokenize(s);
        let mut iter = tokens.iter();
        if let Some(first_tok) = iter.next() {
            match first_tok {
                ParseToken::RepeatSpecial('`', 2) => {
                    let mut content = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('\n', _) => {
                                return Err(crate::ParseError::UnexpectedChar('\n'))
                            }
                            ParseToken::RepeatSpecial(c, n) => {
                                content.push_str(&c.to_string().repeat(*n))
                            }
                            ParseToken::String(s) => content.push_str(s),
                        }
                    }
                    if !content.ends_with("``") {
                        return Err(crate::ParseError::UnexpectedEnd);
                    }
                    content.pop();
                    content.pop();
                    return Ok(Code {
                        content: content.into_boxed_str(),
                        multiline: None,
                    });
                }
                ParseToken::RepeatSpecial('`', 3) => {
                    let mut kind = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('\n', _) => {
                                break;
                            }
                            ParseToken::RepeatSpecial('`', 3) => {
                                return Err(crate::ParseError::EmptyContent)
                            }
                            ParseToken::String(s) => kind.push_str(s),
                            ParseToken::RepeatSpecial(c, n) => {
                                kind.push_str(&c.to_string().repeat(*n))
                            }
                        }
                    }
                    let mut content = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('\n', _) => continue,
                            ParseToken::RepeatSpecial(c, n) => {
                                content.push_str(&c.to_string().repeat(*n))
                            }
                            ParseToken::String(s) => content.push_str(s),
                        }
                    }
                    if !content.ends_with("```") {
                        return Err(crate::ParseError::UnexpectedEnd);
                    }
                    content.pop();
                    content.pop();
                    content.pop();
                    return Ok(Code {
                        content: content.into_boxed_str(),
                        multiline: Some(CodeKind::Unknown(kind.into_boxed_str())),
                    });
                }
                ParseToken::String(s) => {
                    return Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
                ParseToken::RepeatSpecial(c, _) => {
                    return Err(crate::ParseError::UnexpectedChar(*c))
                }
            }
        }
        return Err(crate::ParseError::EmptyDocument);
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let s = if self.multiline.is_some() {
            "```"
        } else {
            "``"
        };
        format!("{}{}{}``", s, self.content, s)
    }
}

impl super::Element for Code {}
