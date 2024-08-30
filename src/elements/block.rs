use std::str::FromStr;

use crate::ParseToken;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeKind {
    Cpp,
    CStandard,
    CSharp,
    Go,
    Haskell,
    Java,
    JavaScript,
    Lua,
    Python,
    Ruby,
    Rust,
    Unknown(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub content: Box<str>,
    pub multiline: Option<CodeKind>,
}

impl FromStr for CodeKind {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csharp" => Ok(CodeKind::CSharp),
            "c" => Ok(CodeKind::CStandard),
            "cpp" => Ok(CodeKind::Cpp),
            "go" => Ok(CodeKind::Go),
            "haskell" => Ok(CodeKind::Haskell),
            "java" => Ok(CodeKind::Java),
            "javascript" => Ok(CodeKind::JavaScript),
            "lua" => Ok(CodeKind::Lua),
            "python" => Ok(CodeKind::Python),
            "ruby" => Ok(CodeKind::Ruby),
            "rust" => Ok(CodeKind::Rust),
            _ => Ok(CodeKind::Unknown(s.to_string().into_boxed_str())),
        }
    }
}

impl ToString for CodeKind {
    fn to_string(&self) -> String {
        match self {
            CodeKind::CSharp => String::from("csharp"),
            CodeKind::CStandard => String::from("c"),
            CodeKind::Cpp => String::from("cpp"),
            CodeKind::Go => String::from("go"),
            CodeKind::Haskell => String::from("haskell"),
            CodeKind::Java => String::from("java"),
            CodeKind::JavaScript => String::from("javascript"),
            CodeKind::Lua => String::from("lua"),
            CodeKind::Python => String::from("python"),
            CodeKind::Ruby => String::from("ruby"),
            CodeKind::Rust => String::from("rust"),
            CodeKind::Unknown(s) => s.to_string(),
        }
    }
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
                    let mut kind: String = String::new();
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
