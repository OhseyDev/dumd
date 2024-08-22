use std::str::FromStr;

use url::Url;

use crate::ParseToken;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub(crate) name: Box<str>,
    pub(crate) href: Url,
    pub(crate) img: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLvl {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Bold(Box<str>),
    BoldItalic(Box<str>),
    Def(Box<str>),
    Italic(Box<str>),
    Link(Link),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heading {
    pub(crate) level: HeadingLvl,
    pub(crate) content: Box<[Item]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph {}

impl HeadingLvl {
    pub fn increment(self) -> Self {
        match self {
            Self::Level1 => Self::Level2,
            Self::Level2 => Self::Level3,
            Self::Level3 => Self::Level4,
            Self::Level4 => Self::Level5,
            Self::Level5 => Self::Level6,
            Self::Level6 => Self::Level6,
        }
    }
    pub fn decrement(self) -> Self {
        match self {
            Self::Level1 => Self::Level1,
            Self::Level2 => Self::Level1,
            Self::Level3 => Self::Level2,
            Self::Level4 => Self::Level3,
            Self::Level5 => Self::Level4,
            Self::Level6 => Self::Level5,
        }
    }
}

macro_rules! into_headinglvl {
    ($num:ident) => {
        impl Into<$num> for HeadingLvl {
            fn into(self) -> $num {
                match self {
                    Self::Level1 => 1,
                    Self::Level2 => 2,
                    Self::Level3 => 3,
                    Self::Level4 => 4,
                    Self::Level5 => 5,
                    Self::Level6 => 6,
                }
            }
        }
    };
}
into_headinglvl!(u8);
into_headinglvl!(u16);
into_headinglvl!(u32);
into_headinglvl!(u64);
into_headinglvl!(u128);
into_headinglvl!(usize);
into_headinglvl!(i8);
into_headinglvl!(i16);
into_headinglvl!(i32);
into_headinglvl!(i64);
into_headinglvl!(i128);
into_headinglvl!(isize);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quote<'a> {
    Nested(&'a Quote<'a>),
    Items(Box<[Item]>),
}

impl Item {
    pub fn asterick(&mut self) {
        *self = match self {
            Self::Italic(c) => Self::Bold(c.clone()),
            Self::Bold(c) => Self::BoldItalic(c.clone()),
            Self::BoldItalic(c) => Self::Bold(c.clone()),
            _ => self.clone(),
        }
    }
    pub fn asterick_cons(self) -> Self {
        match self {
            Self::Italic(c) => Self::Bold(c),
            Self::Bold(c) => Self::BoldItalic(c),
            Self::BoldItalic(c) => Self::Bold(c),
            _ => self,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Italic(c) => c.is_empty(),
            Self::Bold(c) => c.is_empty(),
            Self::BoldItalic(c) => c.is_empty(),
            _ => false,
        }
    }
}

impl FromStr for Item {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = crate::tokenize(s);
        let mut iter = tokens.iter();
        while let Some(first_tok) = iter.next() {
            match first_tok {
                ParseToken::RepeatSpecial('[', 0) => {
                    let name = if let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
                            }
                            ParseToken::String(s) => s.to_owned().into_boxed_str(),
                        }
                    } else {
                        return Err(crate::ParseError::UnexpectedEnd);
                    };
                    if let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial(']', 0) => {}
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
                    if let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('(', 0) => {}
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
                    let mut src = String::new();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial(c, n) => {
                                src.push_str(&c.to_string().repeat(*n + 1))
                            }
                            ParseToken::String(s) => src.push_str(s),
                        }
                    }
                    if !src.ends_with(')') {
                        return Err(crate::ParseError::UnexpectedEnd);
                    } else {
                        src.remove(src.len() - 1);
                    }
                    let u = url::Url::parse(&src);
                    if let Some(e) = u.as_ref().err() {
                        return Err(crate::ParseError::UrlError(e.clone()));
                    }
                    return Ok(Item::Link(Link {
                        name,
                        href: u.ok().unwrap(),
                        img: false,
                    }));
                }
                ParseToken::RepeatSpecial('*', n) => {
                    if *n >= 3 {
                        return Err(crate::ParseError::UnexpectedChar('*'));
                    }
                    let src = if let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
                            }
                            ParseToken::String(s) => s.to_owned(),
                        }
                    } else {
                        return Err(crate::ParseError::UnexpectedEnd);
                    };
                    if let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial('*', m) => {
                                if m > n {
                                    return Err(crate::ParseError::UnexpectedChar('*'));
                                } else if m < n {
                                    return Err(crate::ParseError::UnexpectedEnd);
                                } else {
                                    let mut i = 0;
                                    let mut val = Self::Italic(src.into_boxed_str());
                                    while i < *n {
                                        i += 1;
                                        val.asterick()
                                    }
                                    return Ok(val);
                                }
                            }
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
                            }
                            ParseToken::String(s) => {
                                return Err(crate::ParseError::UnexpectedString(s.clone()))
                            }
                        }
                    }
                }
                ParseToken::String(s) => {
                    let mut src = s.clone();
                    while let Some(tok) = iter.next() {
                        match tok {
                            ParseToken::RepeatSpecial(c, n) => {
                                src.push_str(&c.to_string().repeat(*n + 1))
                            }
                            ParseToken::String(s) => src.push_str(s),
                        }
                    }
                }
                ParseToken::RepeatSpecial(' ', _) => continue,
                _ => {}
            }
        }
        return Err(crate::ParseError::EmptyDocument);
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        return match self {
            Self::Def(s) => s.to_string(),
            Self::Italic(s) => format!("*{}*", s),
            Self::Bold(s) => format!("**{}**", s),
            Self::BoldItalic(s) => format!("***{}***", s),
            Self::Link(l) => format!("{}[{}]({})", if l.img { "!" } else { "" }, l.name, l.href),
        };
    }
}

impl ToString for Heading {
    fn to_string(&self) -> String {
        let mut content = "#".repeat(self.level.into());
        for i in self.content.iter() {
            content.push_str(&i.to_string())
        }
        content
    }
}
