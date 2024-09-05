use crate::elements::text::{Heading, HeadingLvl, Item, Link, LinkSource, Reference};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl super::Builder for ReferenceBuilder {
    type Output = Reference;
    fn build(self) -> Result<Self::Output, super::Error> {
        let url = if let Some(u) = self.href {
            u
        } else {
            return Err(super::Error::IncompleteData);
        };
        if self.name.is_empty() {
            return Err(super::Error::IncompleteData);
        }
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            title: self.title.into_boxed_str(),
            href: url,
        })
    }
    #[allow(refining_impl_trait)]
    fn new() -> Self {
        Self {
            name: String::new(),
            href: None,
            title: String::new(),
        }
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

impl super::Builder for ItemBuilder {
    type Output = Item;
    fn build(self) -> Result<Self::Output, super::Error> {
        match self {
            Self::Bold(s) => Ok(Self::Output::Bold(s.into_boxed_str())),
            Self::BoldItalic(s) => Ok(Self::Output::BoldItalic(s.into_boxed_str())),
            Self::Def(s) => Ok(Self::Output::Def(s.into_boxed_str())),
            Self::Italic(s) => Ok(Self::Output::Italic(s.into_boxed_str())),
            Self::Link(l) => Ok(Self::Output::Link(l)),
            Self::Undefined => Err(super::Error::IncompleteData),
        }
    }
    #[allow(refining_impl_trait)]
    fn new() -> Self {
        ItemBuilder::Undefined
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl super::Builder for LinkBuilder {
    type Output = Link;
    fn build(self) -> Result<Self::Output, super::Error> {
        if self.name.is_empty() {
            return Err(super::Error::IncompleteData);
        }
        if let LinkSource::Ref(s) = &self.src {
            if s.is_empty() {
                return Err(super::Error::IncompleteData);
            }
        };
        Ok(Self::Output {
            name: self.name.into_boxed_str(),
            src: self.src,
            img: self.img,
        })
    }
    #[allow(refining_impl_trait)]
    fn new() -> Self {
        LinkBuilder {
            name: String::new(),
            src: LinkSource::None,
            img: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl super::Builder for HeadingBuilder {
    type Output = Heading;
    fn build(self) -> Result<Self::Output, super::Error> {
        if self.content.is_empty() {
            return Err(super::Error::IncompleteData);
        }
        Ok(Self::Output {
            level: self.level,
            content: self.content,
        })
    }
    #[allow(refining_impl_trait)]
    fn new() -> Self {
        HeadingBuilder {
            content: String::new(),
            level: HeadingLvl::Level1,
        }
    }
}
