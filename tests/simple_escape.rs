#![cfg(feature = "std")]
use lazy_template::{
    enclosed::{Parser, SimpleEscapeParser, SimpleQuery, SimpleQueryParser},
    IntoTemplateSystem,
};
use pretty_assertions::assert_eq;
use std::convert::Infallible;

#[test]
fn make_special_characters() {
    let actual = Parser::curly_braces()
        .with_escape_parser(SimpleEscapeParser)
        .with_query_parser(SimpleQueryParser)
        .into_template_system::<SimpleQuery>()
        .lazy_parse(r"special characters: \\, \0, \b, \e, \n, \r, \t")
        .to_string(Ok::<&str, Infallible>)
        .unwrap();
    dbg!(&actual);
    let expected = "special characters: \\, \0, \u{8}, \u{1b}, \n, \r, \t";
    dbg!(expected);
    assert_eq!(actual, expected);
}

#[test]
fn escape_curly_braces() {
    let map = |query| match query {
        "foo" => Ok(123),
        "bar" => Ok(456),
        other => Err(format!("{other} is undefined")),
    };
    let actual = Parser::curly_braces()
        .with_escape_parser(SimpleEscapeParser)
        .with_query_parser(SimpleQueryParser)
        .into_template_system::<SimpleQuery>()
        .lazy_parse(r"\{foo\} is {foo}, \{bar\} is {bar}")
        .to_string(map)
        .unwrap();
    dbg!(&actual);
    let expected = "{foo} is 123, {bar} is 456";
    dbg!(expected);
    assert_eq!(actual, expected);
}
