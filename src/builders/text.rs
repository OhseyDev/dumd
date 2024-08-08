use crate::md::elements::text::{Heading, HeadingLvl, Item, Link};
use url::Url;

#[derive(Debug)]
pub struct LinkBuilder {
    name: String,
    href: Option<Url>,
    img: bool,
}

#[derive(Debug)]
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

impl super::Builder for ItemBuilder {
    type Output = Item;
    fn build(self) -> Result<Self::Output, super::Error> {
        match self {
            Self::Bold(s) => Ok(Item::Bold(s.into_boxed_str())),
            Self::BoldItalic(s) => Ok(Item::BoldItalic(s.into_boxed_str())),
            Self::Def(s) => Ok(Item::Def(s.into_boxed_str())),
            Self::Italic(s) => Ok(Item::Italic(s.into_boxed_str())),
            Self::Link(l) => Ok(Item::Link(l)),
            Self::Undefined => Err(super::Error::IncompleteData),
        }
    }
    fn new() -> impl super::Builder + Sized {
        ItemBuilder::Undefined
    }
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
        self.href = Some(href);
        self
    }
    pub fn make_img(mut self) -> Self {
        self.img = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingBuilder {
    content: Vec<Item>,
    level: HeadingLvl,
}

impl super::Builder for LinkBuilder {
    type Output = Link;
    fn build(self) -> Result<Self::Output, super::Error> {
        if self.name.is_empty() {
            return Err(super::Error::IncompleteData);
        }
        let href = if let Some(url) = self.href {
            url
        } else {
            return Err(super::Error::IncompleteData);
        };
        Ok(Link {
            name: self.name.into_boxed_str(),
            href,
            img: self.img,
        })
    }
    fn new() -> impl super::Builder {
        LinkBuilder {
            name: String::new(),
            href: None,
            img: false,
        }
    }
}

impl HeadingBuilder {
    pub fn item(mut self, item: Item) -> Self {
        self.content.push(item);
        self
    }
}

impl super::Builder for HeadingBuilder {
    type Output = Heading;
    fn build(self) -> Result<Self::Output, super::Error> {
        if self.content.is_empty() {
            return Err(super::Error::IncompleteData);
        }
        todo!()
    }
    fn new() -> impl super::Builder {
        HeadingBuilder {
            content: Vec::new(),
            level: HeadingLvl::Level1,
        }
    }
}
