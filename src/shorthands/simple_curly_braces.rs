use crate::{
    enclosed::{self, SimpleEnclosedTemplateSystem, SimpleEscapeParser, SimpleQueryParser},
    IntoTemplateSystem,
};

/// Create a simple template of string interpolation with curly braces.
///
/// All queries are enclosed within a pair of curly braces characters:
///
/// ```
/// # #[cfg(not(feature = "std"))] fn main() {}
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// # use lazy_template::simple_curly_braces;
/// let actual = simple_curly_braces()
///     .lazy_parse("foo = {FOO}; bar = {BAR}")
///     .to_string(|query| match query {
///         "FOO" => Ok(123),
///         "BAR" => Ok(456),
///         _ => Err(()),
///     })
///     .unwrap();
/// let expected = "foo = 123; bar = 456";
/// assert_eq!(actual, expected);
/// # }
/// ```
///
/// To prevent the curly braces from being interpreted as a query, simply escape them:
///
/// ```
/// # #[cfg(not(feature = "std"))] fn main() {}
/// # #[cfg(feature = "std")] fn main() {
/// # use pretty_assertions::assert_eq;
/// # use lazy_template::simple_curly_braces;
/// let actual = simple_curly_braces()
///     .lazy_parse(r"foo = \{FOO\}; bar = {BAR}")
///     .to_string(|query| match query {
///         "FOO" => Ok(123),
///         "BAR" => Ok(456),
///         _ => Err(()),
///     })
///     .unwrap();
/// let expected = "foo = {FOO}; bar = 456"; // expect '{FOO}' to not be replaced
/// assert_eq!(actual, expected);
/// # }
/// ```
pub fn simple_curly_braces<'a>() -> SimpleEnclosedTemplateSystem<'a> {
    enclosed::Parser::curly_braces()
        .with_escape_parser(SimpleEscapeParser)
        .with_query_parser(SimpleQueryParser)
        .into_template_system()
}

#[cfg(feature = "std")]
#[test]
fn type_inference() {
    let _ = simple_curly_braces().lazy_parse("").to_string(|query| {
        let _ = query.to_string();
        Ok::<_, ()>(0)
    });
}
