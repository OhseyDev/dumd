use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    content: Box<str>,
    indented: Box<[Box<str>]>,
    ordered: bool,
}

#[derive(Debug, Clone)]
pub enum Element {
    Ordered(Ordered),
    Unordered(Unordered),
}

#[derive(Debug, Clone)]
pub struct Ordered {
    items: Box<[Item]>,
}

#[derive(Debug, Clone)]
pub struct Unordered {
    items: Box<[Item]>,
}

#[derive(Debug, Default, Clone)]
pub struct ItemBuilder {
    content: String,
    indented: Vec<Box<str>>,
    ordered: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Builder {
    items: Vec<Item>,
    ordered: bool,
}

impl super::Element for Ordered {
    fn parse(s: &mut Iter<crate::ParseToken>) -> Result<Self, crate::ParseError> {
        todo!()
    }
}

impl ToString for Ordered {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut i = 1;
        for item in self.items.iter() {
            s.push_str(format!("{}. {}\n", i, item.to_string()).as_str());
            i += 1;
        }
        s.pop();
        return s;
    }
}

impl ToString for Unordered {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for item in self.items.iter() {
            s.push_str(format!(" - {}", item.to_string()).as_str())
        }
        return s;
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        let mut s = self.content.to_string();
        let mut i = 1;
        for item in self.indented.iter() {
            s.push_str(format!("\n\t{}. {}", i, item).as_str());
            i += 1;
        }
        return s;
    }
}

impl ItemBuilder {
    pub fn content(mut self, s: String) -> Self {
        self.content = s;
        self
    }
    pub fn indent(mut self, l: Vec<Box<str>>) -> Self {
        self.indented = l;
        self
    }
    pub fn indent_push(mut self, s: String) -> Self {
        self.indented.push(s.into_boxed_str());
        self
    }
    pub fn ordered(mut self) -> Self {
        self.ordered = true;
        self
    }
    pub fn unordered(mut self) -> Self {
        self.ordered = false;
        self
    }
}

impl crate::Builder for ItemBuilder {
    type Output = Item;
    fn build(self) -> Result<Self::Output, crate::Error> {
        Ok(Self::Output {
            content: self.content.into_boxed_str(),
            indented: self.indented.into_boxed_slice(),
            ordered: self.ordered,
        })
    }
}

impl Builder {
    pub fn push(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }
    pub fn ordered(mut self) -> Self {
        self.ordered = true;
        self
    }
    pub fn unordered(mut self) -> Self {
        self.ordered = false;
        self
    }
}

impl crate::Builder for Builder {
    type Output = Element;
    fn build(self) -> Result<Self::Output, crate::Error> {
        let items = self.items.into_boxed_slice();
        return Ok(match self.ordered {
            true => Self::Output::Ordered(Ordered { items }),
            false => Self::Output::Unordered(Unordered { items }),
        });
    }
}
