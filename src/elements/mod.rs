use std::fmt::Debug;
use std::str::FromStr;

pub mod block;
pub mod list;
pub mod text;

pub trait Element: ToString + FromStr + Debug + PartialEq + PartialOrd + Eq + Ord + Clone {}
