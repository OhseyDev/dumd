use std::fmt::Debug;

pub mod text;

pub trait Builder {
    type Output;
    fn new() -> impl Builder + Sized;
    fn build(self) -> Result<Self::Output, Error>;
}

pub trait Content: Debug {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    IncompleteData,
}
