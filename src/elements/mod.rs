use std::fmt::Debug;
use std::str::FromStr;

pub mod block;
pub mod list;
pub mod text;

pub trait Element: ToString + FromStr + Debug + PartialEq + PartialOrd + Eq + Ord + Clone {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum ParseToken {
    RepeatSpecial(char, usize),
    String(String),
}

pub(self) fn tokenize(s: &str) -> Vec<ParseToken> {
    let mut tokens = Vec::new();
    let mut last_char = None;
    let mut counter: usize = 0;
    for c in s.chars() {
        match c {
            _ => if let Some(lc) = last_char {
                if lc == c {
                    counter += 1;
                } else {
                    tokens.push(ParseToken::RepeatSpecial(c, counter));
                    counter = 0;
                    last_char = Some(c);
                }
            }
        }
    }
    return tokens;
}
