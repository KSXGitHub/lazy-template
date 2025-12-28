use crate::{
    enclosed::{
        self, simple_escape, simple_query, SimpleEnclosedTemplateSystem, SimpleEscapeParser,
        SimpleQueryParser,
    },
    iter::{EagerParseIter, LazyParseIter, ParsedTemplate},
    EnclosedTemplateParser, IntoTemplateSystem, Template, TemplateApplicationError,
};
use core::{convert::Infallible, fmt};

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
pub fn simple_curly_braces() -> SimpleCurlyBraces {
    enclosed::Parser::curly_braces()
        .with_escape_parser(SimpleEscapeParser)
        .with_query_parser(SimpleQueryParser)
        .into_template_system()
}

/// Return type of [`simple_curly_braces`].
pub type SimpleCurlyBraces = SimpleEnclosedTemplateSystem<'static>;

/// Return type of [`SimpleCurlyBraces::lazy_parse`].
pub type LazilyParsed<'a, Query> = Template<
    LazyParseIter<'a, EnclosedTemplateParser<SimpleEscapeParser, SimpleQueryParser>>,
    Query,
>;

pub use LazilyParsed as LazilyParsedTemplate;

#[cfg_attr(
    feature = "std",
    doc = r"Error type of [`LazilyParsedTemplate::to_string`] and [`LazilyParsedTemplate::write_to`]."
)]
#[cfg_attr(
    not(feature = "std"),
    doc = r"Error type of [`LazilyParsedTemplate::write_to`]."
)]
pub type LazilyParsedApplicationError<QueryError> = TemplateApplicationError<
    enclosed::ParseError<simple_escape::ParseError, simple_query::ParseError>,
    QueryError,
    fmt::Error,
>;

/// Value type of [`SimpleCurlyBraces::eager_parse`].
pub type EagerlyParsed<SegmentContainer, Query> = ParsedTemplate<SegmentContainer, Query>;

/// Error type of [`SimpleCurlyBraces::eager_parse`].
pub type EagerParseError =
    enclosed::ParseError<simple_escape::ParseError, simple_query::ParseError>;

/// Return type of [`EagerlyParsed::to_template`].
pub type EagerlyParsedTemplate<SegmentIter, Query> = Template<EagerParseIter<SegmentIter>, Query>;

#[cfg_attr(
    feature = "std",
    doc = r"Error type of [`EagerlyParsedTemplate::to_string`] and [`EagerlyParsedTemplate::write_to`]."
)]
#[cfg_attr(
    not(feature = "std"),
    doc = r"Error type of [`EagerlyParsedTemplate::write_to`]."
)]
pub type EagerlyParsedApplicationError<QueryError> =
    TemplateApplicationError<Infallible, QueryError, fmt::Error>;

#[cfg(feature = "std")]
#[cfg(test)]
mod std_tests {
    use super::{
        simple_curly_braces, EagerParseError, EagerlyParsed, EagerlyParsedApplicationError,
        EagerlyParsedTemplate, LazilyParsed, LazilyParsedApplicationError, SimpleCurlyBraces,
    };
    use derive_more::{Display, Error};

    #[test]
    fn using_type_aliases() {
        fn _type_check() {
            #[derive(Debug, Display, Error, Clone, Copy)]
            enum QueryError {}

            let system: SimpleCurlyBraces = simple_curly_braces();

            let lazy_parsed_template: LazilyParsed<'_, _> = system.lazy_parse("");
            let lazy_result: Result<String, LazilyParsedApplicationError<QueryError>> =
                lazy_parsed_template.to_string(|_| Ok::<_, QueryError>(""));
            drop(lazy_result);

            let eager_parsed_template: Result<
                EagerlyParsed<Vec<crate::enclosed::Segment<&str>>, &str>,
                EagerParseError,
            > = system.eager_parse::<Vec<_>>("");
            let eager_parsed_template: EagerlyParsedTemplate<_, &str> =
                eager_parsed_template.as_ref().unwrap().to_template();
            let eager_result: Result<String, EagerlyParsedApplicationError<QueryError>> =
                eager_parsed_template.to_string(|_| Ok::<_, QueryError>(""));
            drop(eager_result);
        }
    }
}
