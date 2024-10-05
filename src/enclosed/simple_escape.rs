use super::{ComponentParserInput, ParserConfig};
use crate::{Parse, SkipOrFatal};
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
    #[display("Unexpected token {_0:?}")]
    UnexpectedChar(#[error(not(source))] char),
    #[display("Unexpected end of input")]
    UnexpectedEndOfInput,
}

impl<'a> Parse<'a, ParserInput<'a>> for Parser {
    type Output = ParseOutput;
    type Error = SkipOrFatal<(), ParseError>;

    fn parse(&'a self, input: ParserInput<'a>) -> Result<(Self::Output, &'a str), Self::Error> {
        let (head, tail) = split_first_char(input.text).ok_or(SkipOrFatal::Skip(()))?;

        if head != '\\' {
            return Err(SkipOrFatal::Skip(()));
        }

        let (escaped, rest) =
            split_first_char(tail).ok_or(SkipOrFatal::Fatal(ParseError::UnexpectedEndOfInput))?;

        let escaped = map_single_char_escaped(escaped, input.config)
            .ok_or(ParseError::UnexpectedChar(escaped))
            .map_err(SkipOrFatal::Fatal)?;

        Ok((escaped, rest))
    }
}

fn map_single_char_escaped(escaped: char, config: ParserConfig) -> Option<char> {
    if escaped == config.open_bracket || escaped == config.close_bracket {
        return Some(escaped);
    }

    Some(match escaped {
        '\\' => '\\',
        '0' => '\0',
        'b' => '\x08',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => return None,
    })
}
