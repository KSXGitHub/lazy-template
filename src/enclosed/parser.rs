use super::{ComponentParserInput, ParserConfig, Segment};
use crate::{IntoSkipOrFatal, Parse};
use derive_more::{Display, Error};
use split_first_char::split_first_char;

#[derive(Debug, Clone, Copy)]
pub struct EnclosedTemplateParser<EscapeParser, QueryParser> {
    pub config: ParserConfig,
    pub escape_parser: EscapeParser,
    pub query_parser: QueryParser,
}

pub type Parser<EscapeParser, QueryParser> = EnclosedTemplateParser<EscapeParser, QueryParser>;

impl<EscapeParser, QueryParser> Parser<EscapeParser, QueryParser> {
    pub fn with_config(mut self, config: ParserConfig) -> Self {
        self.config = config;
        self
    }

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
    pub fn curly_braces() -> Self {
        Parser {
            config: ParserConfig::curly_braces(),
            escape_parser: (),
            query_parser: (),
        }
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError<ParseEscapeError, ParseQueryError> {
    #[display("Unexpected token {_0:?}")]
    UnexpectedChar(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[display("Fail to escape: {_0}")]
    ParseEscape(ParseEscapeError),
    #[display("Fail to parse query: {_0}")]
    ParseQuery(ParseQueryError),
}

impl<'a, EscapeParser, QueryParser> Parse<'a> for Parser<EscapeParser, QueryParser>
where
    EscapeParser: Parse<'a, ComponentParserInput<'a>>,
    EscapeParser::Output: Into<char>,
    EscapeParser::Error: IntoSkipOrFatal,
    QueryParser: Parse<'a, ComponentParserInput<'a>>,
    QueryParser::Error: IntoSkipOrFatal,
{
    type Output = Segment<QueryParser::Output>;
    type Error = ParseError<
        <EscapeParser::Error as IntoSkipOrFatal>::Fatal,
        <QueryParser::Error as IntoSkipOrFatal>::Fatal,
    >;

    fn parse(&'a self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        let component_parser_input = ComponentParserInput {
            text: input,
            config: self.config,
        };

        let escape_pair = self
            .escape_parser
            .parse_as_component(component_parser_input)
            .map_err(ParseError::ParseEscape)?;
        if let Some((escaped, rest)) = escape_pair {
            return Ok((Segment::Character(escaped.into()), rest));
        }

        let query_pair = self
            .query_parser
            .parse_as_component(component_parser_input)
            .map_err(ParseError::ParseQuery)?;
        if let Some((query, rest)) = query_pair {
            return Ok((Segment::Expression(query), rest));
        }

        let (head, tail) = split_first_char(input).ok_or(ParseError::UnexpectedEndOfInput)?;

        if head == self.config.close_bracket {
            return Err(ParseError::UnexpectedChar(head));
        }

        Ok((Segment::Character(head), tail))
    }
}
