use super::{ComponentParserInput, ParserConfig};
use crate::Parse;
use derive_more::{Display, Error};
use split_first_char::split_first_char;

pub type ParserInput<'a> = ComponentParserInput<'a>;

#[derive(Debug, Clone, Copy)]
pub struct SimpleEscapeParser;
pub type Parser = SimpleEscapeParser;

pub type SimpleEscape = char;
pub type ParseOutput = SimpleEscape;

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum ParseError {
    #[display("Unsupported escape code {_0:?}")]
    UnsupportedEscapeCode(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
}

impl<'a> Parse<'a, ParserInput<'a>> for Parser {
    type Output = ParseOutput;
    type Error = Option<ParseError>;

    fn parse(&self, input: ParserInput<'a>) -> Result<(Self::Output, &'a str), Self::Error> {
        let (head, tail) = split_first_char(input.text).ok_or(None)?;

        if head != '\\' {
            return Err(None);
        }

        let (escaped, rest) =
            split_first_char(tail).ok_or(Some(ParseError::UnexpectedEndOfInput))?;

        let escaped = escape_bracket(escaped, input.config)
            .or_else(|| make_special_character(escaped))
            .ok_or(ParseError::UnsupportedEscapeCode(escaped))
            .map_err(Some)?;

        Ok((escaped, rest))
    }
}

fn escape_bracket(escaped: char, config: ParserConfig) -> Option<char> {
    (escaped == config.open_bracket || escaped == config.close_bracket).then_some(escaped)
}

fn make_special_character(escaped: char) -> Option<char> {
    Some(match escaped {
        '\\' => '\\',
        '0' => '\0',
        'b' => '\x08',
        'e' => '\x1b',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => return None,
    })
}
