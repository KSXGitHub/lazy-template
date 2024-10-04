use super::{simple_query::SimpleQueryParser, ParserConfig, QueryParserInput, Segment};
use crate::Parse;
use derive_more::{Display, Error};
use split_first_char::split_first_char;

#[derive(Debug, Clone, Copy)]
pub struct EnclosedTemplateParser<QueryParser = SimpleQueryParser> {
    pub config: ParserConfig,
    pub query_parser: QueryParser,
}

pub type Parser<QueryParser = SimpleQueryParser> = EnclosedTemplateParser<QueryParser>;

impl<QueryParser> Parser<QueryParser> {
    pub fn curly_braces(query_parser: QueryParser) -> Self {
        Parser {
            config: ParserConfig::curly_braces(),
            query_parser,
        }
    }
}

impl Parser {
    pub fn simple_curly_braces() -> Self {
        Parser::curly_braces(SimpleQueryParser)
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError<ParseQueryError> {
    #[display("Unexpected token {_0:?}")]
    UnexpectedChar(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[display("Fail to parse query: {_0}")]
    ParseQuery(ParseQueryError),
}

impl<'a, QueryParser> Parse<'a> for Parser<QueryParser>
where
    QueryParser: Parse<'a, QueryParserInput<'a>>,
{
    type Output = Segment<QueryParser::Output>;
    type Error = ParseError<QueryParser::Error>;

    fn parse(&'a self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        let (head, tail) = split_first_char(input).ok_or(ParseError::UnexpectedEndOfInput)?;

        if Some(head) == self.config.escape {
            let (escaped, next_tail) =
                split_first_char(tail).ok_or(ParseError::UnexpectedEndOfInput)?;
            return Ok((Segment::Character(escaped), next_tail));
        }

        if head == self.config.open_bracket {
            let query_parser_input = QueryParserInput {
                text: tail,
                config: self.config,
            };
            let (query, next_tail) = self
                .query_parser
                .parse(query_parser_input)
                .map_err(ParseError::ParseQuery)?;
            return Ok((Segment::Expression(query), next_tail));
        }

        if head == self.config.close_bracket {
            return Err(ParseError::UnexpectedChar(head));
        }

        Ok((Segment::Character(head), tail))
    }
}
