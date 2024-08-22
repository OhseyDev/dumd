extern crate dumd;
extern crate url;

use dumd::builders::{self, Builder};
use dumd::elements::text;
use dumd::ParseToken;
use std::str::FromStr;

#[test]
fn parse_basic() {
    assert_eq!(
        Ok(text::Item::Bold("bold text".to_string().into_boxed_str())),
        text::Item::from_str("**bold text**")
    );
    assert_eq!(
        Ok(text::Item::Italic(
            "italic text".to_string().into_boxed_str()
        )),
        text::Item::from_str("*italic text*")
    );
    assert_eq!(
        Ok(text::Item::BoldItalic(
            "bold italic text".to_string().into_boxed_str()
        )),
        text::Item::from_str("***bold italic text***")
    );
    assert_eq!(
        Ok(text::Item::Link(
            builders::text::LinkBuilder::new()
                .name("link".to_string())
                .href(url::Url::parse("https://example.com").expect(""))
                .build()
                .expect("")
        )),
        text::Item::from_str("[link](https://example.com)")
    );
}

#[test]
fn tokenize_test() {
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('*', 1),
            ParseToken::String("bold text".to_string()),
            ParseToken::RepeatSpecial('*', 1)
        ],
        crate::dumd::tokenize("**bold text**")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('[', 0),
            ParseToken::String("link".to_string()),
            ParseToken::RepeatSpecial(']', 0),
            ParseToken::RepeatSpecial('(', 0),
            ParseToken::String("https".to_string()),
            ParseToken::RepeatSpecial(':', 0),
            ParseToken::RepeatSpecial('/', 1),
            ParseToken::String("example".to_string()),
            ParseToken::RepeatSpecial('.', 0),
            ParseToken::String("com".to_string()),
            ParseToken::RepeatSpecial(')', 0),
        ],
        crate::dumd::tokenize("[link](https://example.com)")
    );
}
