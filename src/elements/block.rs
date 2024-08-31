use std::{slice::Iter, str::FromStr};

use crate::ParseToken;

use super::Element;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeKind {
    Cpp(usize),
    CStandard(usize),
    CSharp(usize),
    Go(usize),
    Haskell(usize),
    Java(usize),
    JavaScript(usize),
    Lua(usize),
    Python(usize),
    Ruby(usize),
    Rust(usize),
    None(usize),
    Unknown(Box<str>, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub content: Box<str>,
    pub kind: CodeKind,
}

impl CodeKind {
    pub fn increase_indent(mut self, n: usize) -> Self {
        self = match self {
            CodeKind::CSharp(c) => CodeKind::CSharp(c + n),
            CodeKind::CStandard(c) => CodeKind::CStandard(c + n),
            CodeKind::Cpp(c) => CodeKind::Cpp(c + n),
            CodeKind::Go(c) => CodeKind::Go(c + n),
            CodeKind::Haskell(c) => CodeKind::Haskell(c + n),
            CodeKind::Java(c) => CodeKind::Java(c + n),
            CodeKind::JavaScript(c) => CodeKind::JavaScript(c + n),
            CodeKind::Lua(c) => CodeKind::Lua(c + n),
            CodeKind::Python(c) => CodeKind::Python(c + n),
            CodeKind::Ruby(c) => CodeKind::Ruby(c + n),
            CodeKind::Rust(c) => CodeKind::Rust(c + n),
            CodeKind::None(c) => CodeKind::None(c + n),
            CodeKind::Unknown(s, c) => CodeKind::Unknown(s, c + n),
        };
        self
    }
}

impl FromStr for CodeKind {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(CodeKind::None(0)),
            "csharp" => Ok(CodeKind::CSharp(0)),
            "c" => Ok(CodeKind::CStandard(0)),
            "cpp" => Ok(CodeKind::Cpp(0)),
            "go" => Ok(CodeKind::Go(0)),
            "haskell" => Ok(CodeKind::Haskell(0)),
            "java" => Ok(CodeKind::Java(0)),
            "javascript" => Ok(CodeKind::JavaScript(0)),
            "lua" => Ok(CodeKind::Lua(0)),
            "python" => Ok(CodeKind::Python(0)),
            "ruby" => Ok(CodeKind::Ruby(0)),
            "rust" => Ok(CodeKind::Rust(0)),
            _ => Ok(CodeKind::Unknown(s.to_string().into_boxed_str(), 0)),
        }
    }
}

impl ToString for CodeKind {
    fn to_string(&self) -> String {
        match self {
            CodeKind::CSharp(_) => String::from("csharp"),
            CodeKind::CStandard(_) => String::from("c"),
            CodeKind::Cpp(_) => String::from("cpp"),
            CodeKind::Go(_) => String::from("go"),
            CodeKind::Haskell(_) => String::from("haskell"),
            CodeKind::Java(_) => String::from("java"),
            CodeKind::JavaScript(_) => String::from("javascript"),
            CodeKind::Lua(_) => String::from("lua"),
            CodeKind::Python(_) => String::from("python"),
            CodeKind::Ruby(_) => String::from("ruby"),
            CodeKind::Rust(_) => String::from("rust"),
            CodeKind::None(_) => String::new(),
            CodeKind::Unknown(s, _) => s.to_string(),
        }
    }
}

#[inline]
fn parse_inner(
    iter: &mut core::slice::Iter<'_, ParseToken>,
    n: usize,
) -> Result<Code, crate::ParseError> {
    let mut content = String::new();
    let mut count = 0;
    while let Some(tok) = iter.next() {
        match tok {
            ParseToken::RepeatSpecial('\n', _) => {
                return Err(crate::ParseError::UnexpectedChar('\n'))
            }
            ParseToken::RepeatSpecial('`', c) => {
                if *c < n {
                    content.push('`')
                } else if content.is_empty() {
                    return Err(crate::ParseError::EmptyContent);
                } else {
                    count = n;
                    break;
                }
            }
            ParseToken::RepeatSpecial(c, n) => content.push_str(&c.to_string().repeat(*n)),
            ParseToken::String(s) => content.push_str(s),
        }
    }
    return if count != 0 {
        Ok(Code {
            content: content.into_boxed_str(),
            kind: CodeKind::None(count),
        })
    } else {
        Err(crate::ParseError::UnexpectedEnd)
    };
}

impl FromStr for Code {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_internal(s)
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let s = match &self.kind {
            CodeKind::Cpp(c) => "`".repeat(*c) + "cpp",
            CodeKind::CSharp(c) => "`".repeat(*c) + "csharp",
            CodeKind::CStandard(c) => "`".repeat(*c) + "c",
            CodeKind::Go(c) => "`".repeat(*c) + "go",
            CodeKind::Haskell(c) => "`".repeat(*c) + "haskell",
            CodeKind::Java(c) => "`".repeat(*c) + "java",
            CodeKind::JavaScript(c) => "`".repeat(*c) + "javascript",
            CodeKind::Lua(c) => "`".repeat(*c) + "lua",
            CodeKind::Python(c) => "`".repeat(*c) + "python",
            CodeKind::Ruby(c) => "`".repeat(*c) + "ruby",
            CodeKind::Rust(c) => "`".repeat(*c) + "rust",
            CodeKind::None(c) => "`".repeat(*c),
            CodeKind::Unknown(s, c) => "`".repeat(*c) + s.to_string().as_str(),
        };
        format!("{}{}{}``", s, self.content, s)
    }
}

impl super::Element for Code {
    fn parse(iter: &mut Iter<ParseToken>) -> Result<Self, crate::ParseError> {
        if let Some(first_tok) = iter.next() {
            return match first_tok {
                ParseToken::RepeatSpecial('`', 1) => parse_inner(iter, 1),
                ParseToken::RepeatSpecial('`', 2) => parse_inner(iter, 2),
                ParseToken::RepeatSpecial('`', n) => {
                    let mut kind: String = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('\n', _) => {
                                break;
                            }
                            ParseToken::String(s) => kind.push_str(s),
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
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
                    if content.ends_with("`".repeat(*n).as_str()) {
                        let mut i = *n;
                        while i > 0 {
                            content.pop();
                            i -= 1;
                        }
                        Ok(Code {
                            content: content.into_boxed_str(),
                            kind: CodeKind::from_str(&kind).unwrap().increase_indent(*n),
                        })
                    } else {
                        Err(crate::ParseError::UnexpectedEnd)
                    }
                }
                ParseToken::String(s) => {
                    return Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
                ParseToken::RepeatSpecial(c, _) => {
                    return Err(crate::ParseError::UnexpectedChar(*c))
                }
            };
        }
        return Err(crate::ParseError::EmptyDocument);
    }
}
