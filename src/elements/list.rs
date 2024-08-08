#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element {
    items: Box<[Box<str>]>,
    len: usize,
    ordered: bool,
}
