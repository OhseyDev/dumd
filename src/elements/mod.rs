use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

pub mod block;
pub mod list;
pub mod text;

pub(crate) trait Element:
    ToString + FromStr + Debug + PartialEq + PartialOrd + Eq + Ord + Clone
{
    fn parse(iter: &mut Iter<crate::ParseToken>) -> Result<Self, crate::ParseError>;
    fn from_str_internal(s: &str) -> Result<Self, crate::ParseError> {
        let tokens = crate::tokenize(s);
        let mut iter = tokens.iter();
        let val = {
            let res = Self::parse(&mut iter);
            match res {
                Ok(t) => t,
                Err(e) => return Err(e),
            }
        };
        crate::token_expect!(iter);
        return Ok(val);
    }
}
