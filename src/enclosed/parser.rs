use super::{ComponentParserInput, ParserConfig, Segment};
use crate::{IntoSkipOrFatal, Parse};
use derive_more::{Display, Error};
use split_char_from_str::SplitCharFromStr;

/// Parse a template string whose queries are placed between an opening bracket character and a closing bracket character,
/// (such as [curly braces](crate::simple_curly_braces)).
#[derive(Debug, Clone, Copy)]
pub struct EnclosedTemplateParser<EscapeParser, QueryParser> {
    pub config: ParserConfig,
    pub escape_parser: EscapeParser,
    pub query_parser: QueryParser,
}

pub type Parser<EscapeParser, QueryParser> = EnclosedTemplateParser<EscapeParser, QueryParser>;

impl<EscapeParser, QueryParser> Parser<EscapeParser, QueryParser> {
    /// Replace [`Parser::config`].
    pub fn with_config(mut self, config: ParserConfig) -> Self {
        self.config = config;
        self
    }

    /// Replace [`Parser::escape_parser`].
    pub fn with_escape_parser<NewEscapeParser>(
        self,
        escape_parser: NewEscapeParser,
    ) -> Parser<NewEscapeParser, QueryParser> {
        let Parser {
            config,
            query_parser,
            escape_parser: _,
        } = self;
        Parser {
            config,
            escape_parser,
            query_parser,
        }
    }

    /// Replace [`Parser::query_parser`].
    pub fn with_query_parser<NewQueryParser>(
        self,
        query_parser: NewQueryParser,
    ) -> Parser<EscapeParser, NewQueryParser> {
        let Parser {
            config,
            escape_parser,
            query_parser: _,
        } = self;
        Parser {
            config,
            escape_parser,
            query_parser,
        }
    }
}

impl Parser<(), ()> {
    /// Create a builder of an [`EnclosedTemplateParser`] of templates whose queries should be placed between curly braces.
    ///
    /// The curly braces can be replaced by [replacing the config][Parser::with_config].
    ///
    /// The value returned from this function is not useful immediately. The [query parser][Parser::with_query_parser] and the
    /// [escape parser][Parser::with_escape_parser] must be replaced first.
    ///
    /// **Usage example:**
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))] fn main() {}
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// use lazy_template::{
    ///     enclosed::{Parser, SimpleEscapeParser, SimpleQuery, SimpleQueryParser},
    ///     IntoTemplateSystem,
    /// };
    /// let output = Parser::curly_braces()
    ///     .with_escape_parser(SimpleEscapeParser)
    ///     .with_query_parser(SimpleQueryParser)
    ///     .into_template_system::<SimpleQuery>()
    ///     .lazy_parse("{name} is a {age} years old {descriptor}")
    ///     .to_string(|query| match query {
    ///         "name" => Ok("Alice"),
    ///         "age" => Ok("20"),
    ///         "descriptor" => Ok("girl"),
    ///         _ => Err(format!("Can't answer {query:?}")),
    ///     })
    ///     .unwrap();
    /// assert_eq!(output, "Alice is a 20 years old girl");
    /// # }
    /// ```
    pub fn curly_braces() -> Self {
        Parser {
            config: ParserConfig::curly_braces(),
            escape_parser: (),
            query_parser: (),
        }
    }
}

/// Error type of [`Parse`] on [`EnclosedTemplateParser`].
#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError<ParseEscapeError, ParseQueryError> {
    #[display("Unexpected token {_0:?}")]
    UnexpectedChar(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[display("Failed to escape: {_0}")]
    ParseEscape(ParseEscapeError),
    #[display("Failed to parse query: {_0}")]
    ParseQuery(ParseQueryError),
}

impl<'a, EscapeParser, QueryParser> Parse<'a> for Parser<EscapeParser, QueryParser>
where
    EscapeParser: Parse<'a, ComponentParserInput<'a>, Output = char>,
    EscapeParser::Error: IntoSkipOrFatal,
    QueryParser: Parse<'a, ComponentParserInput<'a>>,
    QueryParser::Error: IntoSkipOrFatal,
{
    type Output = Segment<QueryParser::Output>;
    type Error = ParseError<
        <EscapeParser::Error as IntoSkipOrFatal>::Fatal,
        <QueryParser::Error as IntoSkipOrFatal>::Fatal,
    >;

    fn parse(&self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        let component_parser_input = ComponentParserInput {
            text: input,
            config: self.config,
        };

        let escape_pair = self
            .escape_parser
            .parse_as_component(component_parser_input)
            .map_err(ParseError::ParseEscape)?;
        if let Some((escaped, rest)) = escape_pair {
            return Ok((Segment::Character(escaped), rest));
        }

        let query_pair = self
            .query_parser
            .parse_as_component(component_parser_input)
            .map_err(ParseError::ParseQuery)?;
        if let Some((query, rest)) = query_pair {
            return Ok((Segment::Expression(query), rest));
        }

        let (head, tail) = input
            .split_first_char()
            .ok_or(ParseError::UnexpectedEndOfInput)?;

        if head == self.config.close_bracket {
            return Err(ParseError::UnexpectedChar(head));
        }

        Ok((Segment::Character(head), tail))
    }
}
