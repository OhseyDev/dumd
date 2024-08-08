use std::str::FromStr;

use url::Url;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ItemOp {
    Eval,
    Close,
}

impl FromStr for Item {
    type Err = crate::md::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i: usize = 0;
        let mut chars = s.chars();
        let mut content = String::new();
        let mut last_char: Option<char> = None;
        let mut item: Option<Item> = None;
        let mut op = ItemOp::Eval;
        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if op == ItemOp::Close {
                    } else if let Some(i) = &mut item {
                        if i.is_empty() {
                            if match i {
                                Item::BoldItalic(_) => true,
                                _ => false,
                            } {
                                return Err(crate::md::ParseError::UnexpectedChar('*'));
                            }
                            i.asterick();
                        } else {
                            op = ItemOp::Close;
                        }
                    } else {
                        item = Some(Item::Italic(Box::from("")))
                    }
                }
                '[' => {
                    if let Some(i) = item {
                        let _ = i;
                        todo!();
                    } else if let Some('!') = last_char {
                        todo!()
                    } else {
                        todo!()
                    }
                }
                ']' => todo!(),
                _ => {
                    if let Some(c) = last_char {
                        content.push(c);
                    }
                    last_char = Some(c);
                }
            }
            i += 1;
        }
        let _ = i;
        Err(crate::md::ParseError::UnexpectedEnd)
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
