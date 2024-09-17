use super::{block, text};
use super::Builder;
use std::str::FromStr;

#[test]
fn parse_code() {
    assert_eq!(
        Ok(block::Code {
            content: "code".to_string().into_boxed_str(),
            kind: block::CodeKind::None(2)
        }),
        block::Code::from_str("``code``")
    );
    assert_eq!(
        Ok(block::Code {
            content: "a type of code".to_string().into_boxed_str(),
            kind: block::CodeKind::Unknown("code".to_string().into_boxed_str(), 3),
        }),
        block::Code::from_str("```code\na type of code\n```")
    );
}

#[test]
fn tokenize() {
    use super::ParseToken;
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('*', 2),
            ParseToken::String("bold text".to_string()),
            ParseToken::RepeatSpecial('*', 2),
        ],
        super::tokenize("**bold text**")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('#', 1),
            ParseToken::RepeatSpecial(' ', 1),
            ParseToken::String("Heading 1".to_string()),
        ],
        super::tokenize("# Heading 1")
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
            ParseToken::RepeatSpecial(')', 1)
        ],
        super::tokenize("![link](https://example.com)")
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
        super::tokenize("```code\na type of code\n```")
    );
    assert_eq!(
        vec![
            ParseToken::RepeatSpecial('`', 2),
            ParseToken::String("code".to_string()),
            ParseToken::RepeatSpecial('`', 2)
        ],
        super::tokenize("``code``")
    )
}

#[test]
fn parse_reference() {
    assert_eq!(
        Ok(text::ReferenceBuilder::default()
            .name("1")
            .href(url::Url::parse("https://www.example.com/").unwrap())
            .build()
            .unwrap()),
        text::Reference::from_str("[1]: <https://www.example.com/>")
    );
}

#[test]
fn parse_heading() {
    assert_eq!(
        Ok(text::HeadingBuilder::default()
            .content("Heading 1".to_string())
            .build()
            .unwrap()),
        text::Heading::from_str("# Heading 1"),
    );
}

#[test]
fn parse_text() {
    assert_eq!(
        Ok(text::Item::Bold("bold text".to_string().into_boxed_str())),
        text::Item::from_str("**bold text**")
    );
    assert_eq!(
        Err(crate::ParseError::UnexpectedChar('?')),
        text::Item::from_str("**bold text**?")
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
        Ok(text::LinkBuilder::default()
            .name("link".to_string())
            .href(url::Url::parse("https://example.com").expect(""))
            .build()
            .expect("")
            .into()),
        text::Item::from_str("[link](https://example.com)")
    );
    assert_eq!(
        Ok(text::Item::Link(
            text::LinkBuilder::default()
                .name("link".to_string())
                .href(url::Url::parse("https://example.com").expect(""))
                .make_img()
                .build()
                .expect("")
        )),
        text::Item::from_str("![link](https://example.com)")
    );
}
