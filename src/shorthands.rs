use crate::enclosed::{self, SimpleEnclosedTemplate};
use pipe_trait::Pipe;

pub fn simple_curly_braces<'a>() -> SimpleEnclosedTemplate<'a> {
    enclosed::Parser::simple_curly_braces().pipe(SimpleEnclosedTemplate::new)
}

#[cfg(feature = "std")]
#[test]
fn test_simple_curly_braces() {
    use pretty_assertions::assert_eq;
    let actual = simple_curly_braces()
        .to_string("foo = {FOO}; bar = {BAR}", |query| match query.as_str() {
            "FOO" => Ok(123),
            "BAR" => Ok(456),
            _ => Err(()),
        })
        .unwrap();
    let expected = "foo = 123; bar = 456";
    assert_eq!(actual, expected);
}
