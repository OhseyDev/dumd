use std::{slice::Iter, str::FromStr};

use url::Url;

use crate::{elements::Element, ParseToken};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkSource {
    Url(Url),
    Ref(Box<str>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference {
    pub(crate) name: Box<str>,
    pub(crate) title: Box<str>,
    pub(crate) href: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub(crate) name: Box<str>,
    pub(crate) src: LinkSource,
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
    pub(crate) content: String,
}

impl Element for Heading {
    fn parse(iter: &mut Iter<crate::ParseToken>) -> Result<Self, crate::ParseError> {
        if let Some(t) = iter.next() {
            return match t {
                ParseToken::RepeatSpecial('#', n) => {
                    let tok = if let Some(s) = iter.next() {
                        s
                    } else {
                        return Err(crate::ParseError::UnexpectedEnd);
                    };
                    if *n > 6 {
                        return Err(crate::ParseError::UnexpectedChar('#'));
                    }
                    match tok {
                        ParseToken::RepeatSpecial(' ', _) => {
                            let t = if let Some(t) = iter.next() {
                                t
                            } else {
                                return Err(crate::ParseError::UnexpectedEnd);
                            };
                            match t {
                                ParseToken::RepeatSpecial(c, _) => {
                                    Err(crate::ParseError::UnexpectedChar(*c))
                                }
                                ParseToken::String(s) => Ok(Heading {
                                    level: HeadingLvl::Level1,
                                    content: s.clone(),
                                }),
                            }
                        }
                        ParseToken::RepeatSpecial(c, _) => {
                            Err(crate::ParseError::UnexpectedChar(*c))
                        }
                        ParseToken::String(s) => Ok(Heading {
                            level: HeadingLvl::Level1,
                            content: s.clone(),
                        }),
                    }
                }
                ParseToken::RepeatSpecial(c, _) => Err(crate::ParseError::UnexpectedChar(*c)),
                ParseToken::String(s) => Err(crate::ParseError::UnexpectedString(s.clone())),
            };
        }
        return Err(crate::ParseError::UnexpectedEnd);
    }
}

impl FromStr for Heading {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_internal(s)
    }
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

macro_rules! into_headinglvlf {
    ($num:ident) => {
        impl Into<$num> for HeadingLvl {
            fn into(self) -> $num {
                match self {
                    Self::Level1 => 1.0,
                    Self::Level2 => 2.0,
                    Self::Level3 => 3.0,
                    Self::Level4 => 4.0,
                    Self::Level5 => 5.0,
                    Self::Level6 => 6.0,
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
into_headinglvlf!(f32);
into_headinglvlf!(f64);

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
impl super::Element for Item {
    fn parse(iter: &mut Iter<ParseToken>) -> Result<Self, crate::ParseError> {
        while let Some(first_tok) = iter.next() {
            match first_tok {
                ParseToken::RepeatSpecial('!', 1) => return process_link_item(iter, true),
                ParseToken::RepeatSpecial('[', 1) => return process_link_item(iter, false),
                ParseToken::RepeatSpecial('*', n) => {
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
                                if m < n {
                                    return Err(crate::ParseError::UnexpectedEnd);
                                } else {
                                    let mut i = 1;
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
                                src.push_str(&c.to_string().repeat(*n))
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

#[inline]
fn process_link_item(iter: &mut Iter<ParseToken>, img: bool) -> Result<Item, crate::ParseError> {
    if img {
        let tok = if let Some(n) = iter.next() {
            n
        } else {
            return Err(crate::ParseError::UnexpectedEnd);
        };
        match tok {
            ParseToken::RepeatSpecial('[', 1) => {}
            ParseToken::String(s) => return Err(crate::ParseError::UnexpectedString(s.to_owned())),
            ParseToken::RepeatSpecial(c, _) => return Err(crate::ParseError::UnexpectedChar(*c)),
        }
    }
    let name = if let Some(tok) = iter.next() {
        match tok {
            ParseToken::RepeatSpecial(c, _) => return Err(crate::ParseError::UnexpectedChar(*c)),
            ParseToken::String(s) => s.to_owned().into_boxed_str(),
        }
    } else {
        return Err(crate::ParseError::UnexpectedEnd);
    };
    crate::token_expect_char!(iter, ']', 1);
    crate::token_expect_char!(iter, '(', 1);
    let mut src_str = String::new();
    let res = loop {
        let tok = if let Some(tok) = iter.next() {
            tok
        } else {
            break false;
        };
        match tok {
            ParseToken::RepeatSpecial(')', 1) => break true,
            ParseToken::RepeatSpecial(c, n) => src_str.push_str(&c.to_string().repeat(*n)),
            ParseToken::String(s) => src_str.push_str(s),
        }
    };
    if !res {
        return Err(crate::ParseError::UnexpectedEnd);
    } else if let Some(t) = iter.next() {
        return Err(match t {
            ParseToken::RepeatSpecial(c, _) => crate::ParseError::UnexpectedChar(*c),
            ParseToken::String(s) => crate::ParseError::UnexpectedString(s.to_owned()),
        });
    }
    let u = url::Url::parse(&src_str);
    let src = if let Some(e) = u.as_ref().err() {
        if img {
            return Err(crate::ParseError::InvalidUrl(e.to_owned()));
        }
        for c in src_str.chars() {
            match c {
                '0'..='9' => {}
                'A'..='Z' => {}
                'a'..='z' => {}
                '?' => {}
                '!' => {}
                '.' => {}
                ',' => {}
                ';' => {}
                ':' => {}
                '\'' => {}
                '"' => {}
                _ => return Err(crate::ParseError::InvalidUrl(e.clone())),
            }
        }
        LinkSource::Ref(src_str.into_boxed_str())
    } else {
        LinkSource::Url(u.ok().unwrap())
    };
    return Ok(Item::Link(Link { name, src, img }));
}

impl FromStr for Item {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_internal(s)
    }
}

impl super::Element for Reference {
    fn parse(iter: &mut Iter<ParseToken>) -> Result<Self, crate::ParseError> {
        return if let Some(t) = iter.next() {
            match t {
                ParseToken::RepeatSpecial('[', 1) => {
                    let mut name = String::new();
                    while let Some(t) = iter.next() {
                        match t {
                            ParseToken::String(s) => name.push_str(s),
                            ParseToken::RepeatSpecial('!', n) => name.push_str(&"!".repeat(*n)),
                            ParseToken::RepeatSpecial('?', n) => name.push_str(&"?".repeat(*n)),
                            ParseToken::RepeatSpecial('.', n) => name.push_str(&".".repeat(*n)),
                            ParseToken::RepeatSpecial(',', n) => name.push_str(&",".repeat(*n)),
                            ParseToken::RepeatSpecial(';', n) => name.push_str(&";".repeat(*n)),
                            ParseToken::RepeatSpecial(':', n) => name.push_str(&":".repeat(*n)),
                            ParseToken::RepeatSpecial('\'', n) => name.push_str(&"'".repeat(*n)),
                            ParseToken::RepeatSpecial('"', n) => name.push_str(&"\"".repeat(*n)),
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
                            }
                        }
                    }
                    if name.is_empty() {
                        return Err(crate::ParseError::UnexpectedEnd);
                    }
                    crate::token_expect_char!(iter, ']', 1);
                    crate::token_expect_char!(iter, ':', 1);
                    let t = crate::token_ignore_char!(iter, ' ');
                    let (t, n) = crate::token_ignore_char_restricted1!(iter, '<', 1, t);
                    let url = url::Url::parse(&crate::token_combine_except!(
                        iter,
                        ParseToken::RepeatSpecial('>', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar('>'));
                            }
                            break;
                        }
                    ));
                    if let Some(e) = url.err() {
                        return Err(crate::ParseError::InvalidUrl(e));
                    }

                    todo!()
                }
                ParseToken::RepeatSpecial(c, _) => Err(crate::ParseError::UnexpectedChar(*c)),
                ParseToken::String(s) => Err(crate::ParseError::UnexpectedString(s.to_owned())),
            }
        } else {
            Err(crate::ParseError::EmptyDocument)
        };
    }
}

impl FromStr for Reference {
    type Err = crate::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_internal(s)
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        return match self {
            Self::Def(s) => s.to_string(),
            Self::Italic(s) => format!("*{}*", s),
            Self::Bold(s) => format!("**{}**", s),
            Self::BoldItalic(s) => format!("***{}***", s),
            Self::Link(l) => format!(
                "{}[{}]({})",
                if l.img { "!" } else { "" },
                l.name,
                l.src.to_string()
            ),
        };
    }
}

impl ToString for Heading {
    fn to_string(&self) -> String {
        let mut content = "#".repeat(self.level.into());
        content.push(' ');
        content.push_str(&self.content);
        content
    }
}

impl ToString for LinkSource {
    fn to_string(&self) -> String {
        match self {
            LinkSource::None => String::new(),
            LinkSource::Ref(r) => r.to_string(),
            LinkSource::Url(u) => u.to_string(),
        }
    }
}

impl ToString for Reference {
    fn to_string(&self) -> String {
        let mut s = format!("[{}]: <{}>", self.name, self.href.to_string());
        if !self.title.is_empty() {
            s.push_str(&format!("({})", self.title))
        }
        return s;
    }
}
