extern crate dumd;
extern crate url;

use std::str::FromStr;
use dumd::elements::text;
use dumd::builders::{Builder, self};

#[test]
fn parse_basic() {
    assert_eq!(Ok(text::Item::Bold("bold text".to_string().into_boxed_str())), text::Item::from_str("**bold text**"));
    assert_eq!(Ok(text::Item::Bold("italic text".to_string().into_boxed_str())), text::Item::from_str("*italic text*"));
    assert_eq!(Ok(text::Item::Bold("bold italic text".to_string().into_boxed_str())), text::Item::from_str("***bold italic text***"));
    assert_eq!(Ok(text::Item::Link(builders::text::LinkBuilder::new().name("link".to_string()).href(url::Url::parse("https://example.com").expect("")).build().expect(""))), text::Item::from_str("[link](https://example.com"));
}