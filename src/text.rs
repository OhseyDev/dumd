use std::slice::Iter;

use url::Url;

use crate::{Element, ParseToken};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paragraph {}

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

impl ToString for Heading {
    fn to_string(&self) -> String {
        let mut content = "#".repeat(self.level.into());
        content.push(' ');
        content.push_str(&self.content);
        content
    }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkSource {
    Url(Url),
    Ref(Box<str>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub(crate) name: Box<str>,
    pub(crate) src: LinkSource,
    pub(crate) img: bool,
}

impl Into<Item> for Link {
    fn into(self) -> Item {
        Item::Link(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Bold(Box<str>),
    BoldItalic(Box<str>),
    Def(Box<str>),
    Italic(Box<str>),
    Link(Link),
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
    crate::token_expect!(iter, ']', 1);
    crate::token_expect!(iter, '(', 1);
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference {
    pub(crate) name: Box<str>,
    pub(crate) title: Box<str>,
    pub(crate) href: Url,
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
                            ParseToken::RepeatSpecial('0', n) => name.push_str(&"0".repeat(*n)),
                            ParseToken::RepeatSpecial('1', n) => name.push_str(&"1".repeat(*n)),
                            ParseToken::RepeatSpecial('2', n) => name.push_str(&"2".repeat(*n)),
                            ParseToken::RepeatSpecial('3', n) => name.push_str(&"3".repeat(*n)),
                            ParseToken::RepeatSpecial('4', n) => name.push_str(&"4".repeat(*n)),
                            ParseToken::RepeatSpecial('5', n) => name.push_str(&"5".repeat(*n)),
                            ParseToken::RepeatSpecial('6', n) => name.push_str(&"6".repeat(*n)),
                            ParseToken::RepeatSpecial('7', n) => name.push_str(&"7".repeat(*n)),
                            ParseToken::RepeatSpecial('8', n) => name.push_str(&"8".repeat(*n)),
                            ParseToken::RepeatSpecial('9', n) => name.push_str(&"9".repeat(*n)),
                            ParseToken::RepeatSpecial(']', 1) => {
                                if name.is_empty() {
                                    return Err(crate::ParseError::UnexpectedChar(']'));
                                }
                                break;
                            }
                            ParseToken::RepeatSpecial(c, _) => {
                                return Err(crate::ParseError::UnexpectedChar(*c))
                            }
                        }
                    }
                    if name.is_empty() {
                        return Err(crate::ParseError::UnexpectedEnd);
                    }
                    crate::token_expect!(iter, ':', 1);
                    let t = crate::token_ignore_char!(iter, ' ');
                    let (t, n) = crate::token_ignore_char!(iter, '<', 1, t);
                    let href = &crate::token_combine_except!(
                        t,
                        iter,
                        ParseToken::RepeatSpecial('>', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar('>'));
                            }
                            break;
                        },
                        ParseToken::RepeatSpecial(' ', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar(' '));
                            }
                            break;
                        }
                    );
                    let url = {
                        let url = url::Url::parse(&href);
                        if let Some(e) = url.as_ref().err() {
                            return Err(crate::ParseError::InvalidUrl(e.clone()));
                        }
                        url.unwrap()
                    };
                    let t = crate::token_ignore_char!(
                        iter,
                        ' ',
                        return Ok(Reference {
                            name: name.into_boxed_str(),
                            title: String::new().into_boxed_str(),
                            href: url,
                        })
                    );
                    let (t, n) = crate::token_ignore_char!(iter, '"', 1, t);
                    let t = if n == 0 {
                        crate::token_ignore_char!(iter, '(', 1, t).0
                    } else {
                        t
                    };
                    let title = crate::token_combine_except!(
                        t,
                        iter,
                        ParseToken::RepeatSpecial(')', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar(')'));
                            }
                            break;
                        },
                        ParseToken::RepeatSpecial('(', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar('('));
                            }
                            break;
                        },
                        ParseToken::RepeatSpecial('"', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar('"'));
                            }
                            break;
                        },
                        ParseToken::RepeatSpecial('\'', 1),
                        {
                            if n == 0 {
                                return Err(crate::ParseError::UnexpectedChar('\''));
                            }
                            break;
                        }
                    );
                    return Ok(Reference {
                        name: name.into_boxed_str(),
                        title: title.into_boxed_str(),
                        href: url,
                    });
                }
                ParseToken::RepeatSpecial(c, _) => Err(crate::ParseError::UnexpectedChar(*c)),
                ParseToken::String(s) => Err(crate::ParseError::UnexpectedString(s.to_owned())),
            }
        } else {
            Err(crate::ParseError::EmptyDocument)
        };
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferenceBuilder {
    name: String,
    href: Option<url::Url>,
    title: String,
}

impl ReferenceBuilder {
    pub fn name(mut self, s: &str) -> Self {
        self.name = s.to_string();
        self
    }
    pub fn title(mut self, s: &str) -> Self {
        self.title = s.to_string();
        self
    }
    pub fn href(mut self, u: url::Url) -> Self {
        self.href = Some(u);
        self
    }
}

impl crate::Builder for ReferenceBuilder {
    type Output = Reference;
    fn build(self) -> Result<Self::Output, crate::Error> {
        let url = if let Some(u) = self.href {
            u
        } else {
            return Err(crate::Error::IncompleteData);
        };
        if self.name.is_empty() {
            return Err(crate::Error::IncompleteData);
        }
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            title: self.title.into_boxed_str(),
            href: url,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemBuilder {
    Bold(String),
    BoldItalic(String),
    Def(String),
    Italic(String),
    Link(Link),
    Undefined,
}

impl ItemBuilder {
    pub fn bold(self) -> Self {
        match self {
            Self::Bold(s) => Self::Def(s),
            Self::BoldItalic(s) => Self::Italic(s),
            Self::Italic(s) => Self::BoldItalic(s),
            Self::Def(s) => Self::Bold(s),
            _ => Self::Undefined,
        }
    }
    pub fn italic(self) -> Self {
        match self {
            Self::Bold(s) => Self::BoldItalic(s),
            Self::BoldItalic(s) => Self::Bold(s),
            Self::Italic(s) => Self::Def(s),
            Self::Def(s) => Self::Italic(s),
            _ => Self::Undefined,
        }
    }
    pub fn link(self, l: Link) -> Self {
        Self::Link(l)
    }
    pub fn content(self, s: String) -> Self {
        match self {
            Self::Bold(_) => Self::Bold(s),
            Self::BoldItalic(_) => Self::BoldItalic(s),
            Self::Italic(_) => Self::Italic(s),
            _ => Self::Def(s),
        }
    }
}

impl crate::Builder for ItemBuilder {
    type Output = Item;
    fn build(self) -> Result<Self::Output, crate::Error> {
        match self {
            Self::Bold(s) => Ok(Self::Output::Bold(s.into_boxed_str())),
            Self::BoldItalic(s) => Ok(Self::Output::BoldItalic(s.into_boxed_str())),
            Self::Def(s) => Ok(Self::Output::Def(s.into_boxed_str())),
            Self::Italic(s) => Ok(Self::Output::Italic(s.into_boxed_str())),
            Self::Link(l) => Ok(Self::Output::Link(l)),
            Self::Undefined => Err(crate::Error::IncompleteData),
        }
    }
}

impl Default for ItemBuilder {
    fn default() -> Self {
        ItemBuilder::Undefined
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkBuilder {
    name: String,
    src: LinkSource,
    img: bool,
}

impl LinkBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn name_push(&mut self, c: char) {
        self.name.push(c);
    }
    pub fn name_push_str(&mut self, s: &str) {
        self.name.push_str(s);
    }
    pub fn href(mut self, href: url::Url) -> Self {
        self.src = LinkSource::Url(href);
        self
    }
    pub fn reference(mut self, r: Reference) -> Self {
        self.src = LinkSource::Ref(r.name.clone());
        self
    }
    pub fn make_img(mut self) -> Self {
        self.img = true;
        self
    }
}

impl crate::Builder for LinkBuilder {
    type Output = Link;
    fn build(self) -> Result<Self::Output, crate::Error> {
        if self.name.is_empty() {
            return Err(crate::Error::IncompleteData);
        }
        if let LinkSource::Ref(s) = &self.src {
            if s.is_empty() {
                return Err(crate::Error::IncompleteData);
            }
        };
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            src: self.src,
            img: self.img,
        })
    }
}

impl Default for LinkSource {
    fn default() -> Self {
        Self::None
    }
}

impl Default for HeadingLvl {
    fn default() -> Self {
        Self::Level1
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingBuilder {
    content: String,
    level: HeadingLvl,
}

impl HeadingBuilder {
    pub fn content(mut self, s: String) -> Self {
        self.content = s;
        self
    }
    pub fn level(mut self, l: HeadingLvl) -> Self {
        self.level = l;
        self
    }
}

impl crate::Builder for HeadingBuilder {
    type Output = Heading;
    fn build(self) -> Result<Self::Output, crate::Error> {
        if self.content.is_empty() {
            return Err(crate::Error::IncompleteData);
        }
        Ok(Self::Output {
            level: self.level,
            content: self.content,
        })
    }
}

crate::impl_from_str!(Heading);
crate::impl_from_str!(Item);
crate::impl_from_str!(Reference);
