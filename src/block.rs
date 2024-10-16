use std::{slice::Iter, str::FromStr};

use crate::ParseToken;

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
            CodeKind::Cpp(c) => CodeKind::Cpp(c + n),
            CodeKind::CSharp(c) => CodeKind::CSharp(c + n),
            CodeKind::CStandard(c) => CodeKind::CStandard(c + n),
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
    pub fn get_index(&self) -> usize {
        match self {
            &Self::Cpp(c) => c,
            &Self::CSharp(c) => c,
            &Self::CStandard(c) => c,
            &Self::Go(c) => c,
            &Self::Haskell(c) => c,
            &Self::Java(c) => c,
            &Self::JavaScript(c) => c,
            &Self::Lua(c) => c,
            &Self::Python(c) => c,
            &Self::Ruby(c) => c,
            &Self::Rust(c) => c,
            &Self::None(c) => c,
            &Self::Unknown(_, c) => c,
        }
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
            ParseToken::Number(_, _) => content.push_str(&tok.to_string()),
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

impl ToString for Code {
    fn to_string(&self) -> String {
        let prefix = "`".repeat(self.kind.get_index());
        format!(
            "{}{}{}{}",
            prefix,
            self.kind.to_string(),
            self.content,
            prefix
        )
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
                            ParseToken::Number(_, _) => {
                                return Err(crate::ParseError::UnexpectedString(tok.to_string()))
                            }
                        }
                    }
                    let mut content = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('\n', _) => continue,
                            ParseToken::RepeatSpecial('`', m) => {
                                if n <= m {
                                    break;
                                }
                                content.push_str(&"`".repeat(*m));
                            }
                            ParseToken::RepeatSpecial(c, n) => {
                                content.push_str(&c.to_string().repeat(*n))
                            }
                            ParseToken::String(s) => content.push_str(s),
                            ParseToken::Number(_, _) => {
                                content.push_str(&tok.to_string());
                            }
                        }
                    }
                    Ok(Code {
                        content: content.into_boxed_str(),
                        kind: CodeKind::from_str(&kind).unwrap().increase_indent(*n),
                    })
                }
                ParseToken::String(s) => {
                    return Err(crate::ParseError::UnexpectedString(s.to_owned()))
                }
                ParseToken::RepeatSpecial(c, _) => {
                    return Err(crate::ParseError::UnexpectedChar(*c))
                }
                ParseToken::Number(_, _) => {
                    return Err(crate::ParseError::UnexpectedString(first_tok.to_string()))
                }
            };
        }
        return Err(crate::ParseError::EmptyDocument);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeBuilder {
    content: String,
    kind: Option<CodeKind>,
}

impl CodeBuilder {
    pub fn content(mut self, c: &str) -> Self {
        self.content = c.to_string();
        self
    }
    pub fn kind(mut self, k: CodeKind) -> Self {
        self.kind = Some(k);
        self
    }
}

impl crate::Builder for CodeBuilder {
    type Output = Code;

    fn build(self) -> Result<Self::Output, crate::Error> {
        let kind = if let Some(k) = self.kind {
            k
        } else {
            return Err(crate::Error::IncompleteData);
        };
        Ok(Code {
            content: self.content.into_boxed_str(),
            kind,
        })
    }
}

impl Default for CodeBuilder {
    fn default() -> Self {
        Self {
            content: String::new(),
            kind: None,
        }
    }
}

crate::impl_from_str!(Code);
