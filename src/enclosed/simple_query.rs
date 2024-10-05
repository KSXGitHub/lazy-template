use super::ComponentParserInput;
use crate::Parse;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use split_first_char::split_first_char;

pub type ParserInput<'a> = ComponentParserInput<'a>;

#[derive(Debug, Clone, Copy)]
pub struct SimpleQueryParser;
pub type Parser = SimpleQueryParser;

pub type SimpleQuery<'a> = &'a str;
pub type ParseOutput<'a> = SimpleQuery<'a>;

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError {
    #[display("Unexpected token {_0:?}")]
    UnexpectedChar(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
}

impl<'a> Parse<'a, ParserInput<'a>> for Parser {
    type Output = ParseOutput<'a>;
    type Error = Option<ParseError>;

    fn parse(&'a self, input: ParserInput<'a>) -> Result<(Self::Output, &'a str), Self::Error> {
        let (head, tail) = split_first_char(input.text).ok_or(None)?;

        if head == input.config.close_bracket {
            return head.pipe(ParseError::UnexpectedChar).pipe(Some).pipe(Err);
        }

        if head != input.config.open_bracket {
            return Err(None);
        }

        let (close_index, _) = tail
            .char_indices()
            .find(|(_, char)| *char == input.config.close_bracket)
            .ok_or(ParseError::UnexpectedEndOfInput)
            .map_err(Some)?;
        let query = &tail[..close_index];
        let rest = &tail[(close_index + 1)..];
        Ok((query, rest))
    }
}
