use super::{EnclosedTemplateParser, QueryParserInput};
use crate::{Parse, TemplateSystem};
use derive_more::{AsRef, Deref, Display, Error, Into};

pub type ParserInput<'a> = QueryParserInput<'a>;

#[derive(Debug, Clone, Copy)]
pub struct SimpleQueryParser;
pub type Parser = SimpleQueryParser;

#[derive(Debug, Display, AsRef, Deref, Into, Clone, Copy)]
pub struct SimpleQuery<'a>(&'a str);
pub type ParseOutput<'a> = SimpleQuery<'a>;

impl<'a> SimpleQuery<'a> {
    pub fn as_str(&self) -> &'a str {
        self
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError {
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
}

impl<'a> Parse<'a, ParserInput<'a>> for Parser {
    type Output = ParseOutput<'a>;
    type Error = ParseError;

    fn parse(&'a self, input: ParserInput<'a>) -> Result<(Self::Output, &'a str), Self::Error> {
        let (close_index, _) = input
            .text
            .char_indices()
            .find(|(_, char)| *char == input.config.close_bracket)
            .ok_or(ParseError::UnexpectedEndOfInput)?;
        let query = &input.text[..close_index];
        let rest = &input.text[(close_index + 1)..];
        Ok((SimpleQuery(query), rest))
    }
}

pub type SimpleEnclosedTemplate<'a> =
    TemplateSystem<EnclosedTemplateParser<SimpleQueryParser>, SimpleQuery<'a>>;
