extern crate dumd;
extern crate url;

use dumd::builders::{self, Builder};
use dumd::elements::{block, text};
use dumd::ParseToken;
use std::str::FromStr;

#[test]
fn parse_code() {
    assert_eq!(
        Ok(block::Code {
            content: "code".to_string().into_boxed_str(),
            multiline: None
        }),
        block::Code::from_str("``code``")
    );
    assert_eq!(
        Ok(block::Code {
            content: "a type of code".to_string().into_boxed_str(),
            multiline: Some(block::CodeKind::Unknown(
                "code".to_string().into_boxed_str()
            )),
        }),
        block::Code::from_str("```code\na type of code\n```")
    );
}

#[test]
fn parse_text() {
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
    assert_eq!(
        Ok(text::Item::Link(
            builders::text::LinkBuilder::new()
                .name("link".to_string())
                .href(url::Url::parse("https://example.com").expect(""))
                .make_img()
                .build()
                .expect("")
        )),
        text::Item::from_str("![link](https://example.com)")
    );
}

#[test]
fn tokenize() {
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('*', 2),
            ParseToken::String("bold text".to_string()),
            ParseToken::RepeatSpecial('*', 2)
        ],
        crate::dumd::tokenize("**bold text**")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('!', 1),
            ParseToken::RepeatSpecial('[', 1),
            ParseToken::String("link".to_string()),
            ParseToken::RepeatSpecial(']', 1),
            ParseToken::RepeatSpecial('(', 1),
            ParseToken::String("https".to_string()),
            ParseToken::RepeatSpecial(':', 1),
            ParseToken::RepeatSpecial('/', 2),
            ParseToken::String("example".to_string()),
            ParseToken::RepeatSpecial('.', 1),
            ParseToken::String("com".to_string()),
            ParseToken::RepeatSpecial(')', 1),
        ],
        crate::dumd::tokenize("![link](https://example.com)")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('`', 2),
            ParseToken::String("code".to_string()),
            ParseToken::RepeatSpecial('`', 2)
        ],
        crate::dumd::tokenize("``code``")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('`', 3),
            ParseToken::String("code".to_string()),
            ParseToken::RepeatSpecial('\n', 1),
            ParseToken::String("a type of code".to_string()),
            ParseToken::RepeatSpecial('\n', 1),
            ParseToken::RepeatSpecial('`', 3)
        ],
        crate::dumd::tokenize("```code\na type of code\n```")
    );
}
