extern crate dumd;
extern crate url;

use dumd::builders::{self, Builder};
use dumd::elements::{block, text};
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
fn parse_heading() {
    assert_eq!(
        Ok(builders::text::HeadingBuilder::new()
            .content("Heading 1".to_string())
            .build()
            .unwrap()),
        text::Heading::from_str("# Heading 1"),
    )
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
