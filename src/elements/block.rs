use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    content: String,
    multiline: bool,
}

impl FromStr for Code {
    type Err = crate::md::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r: u8 = 0;
        let mut chars = s.chars();
        let c = loop {
            let c = if let Some(c) = chars.next() {
                c
            } else {
                break '\0';
            };
            match c {
                '`' => {
                    r += 1;
                    if r == 3 {
                        break c;
                    }
                }
                _ => break c,
            }
        };
        let mut b: u8 = 0;
        let mut content = String::new();
        if c == '\0' {
            return Err(crate::md::ParseError::UnexpectedEnd);
        } else if c != '`' {
            content.push(c);
        }
        while let Some(c) = chars.next() {
            match c {
                '`' => {
                    b += 1;
                    if b == r {
                        break;
                    }
                }
                _ => content.push(c),
            }
        }
        if content.is_empty() {
            return Err(crate::md::ParseError::EmptyContent);
        }
        return Ok(Code {
            content,
            multiline: b == 3,
        });
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let s = if self.multiline { "``" } else { "```" };
        format!("{}{}{}``", s, self.content, s)
    }
}

impl super::Element for Code {}
