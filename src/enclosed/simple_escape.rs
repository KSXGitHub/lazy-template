use super::{ComponentParserInput, ParserConfig};
use crate::Parse;
use derive_more::{Display, Error};
use split_char_from_str::SplitCharFromStr;

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
        let (head, tail) = input.text.split_first_char().ok_or(None)?;

        if head != '\\' {
            return Err(None);
        }

        let (escape_code, rest) = tail
            .split_first_char()
            .ok_or(Some(ParseError::UnexpectedEndOfInput))?;

        let char = escape_bracket(escape_code, input.config)
            .or_else(|| make_special_character(escape_code))
            .ok_or(ParseError::UnsupportedEscapeCode(escape_code))
            .map_err(Some)?;

        Ok((char, rest))
    }
}

fn escape_bracket(escape_code: char, config: ParserConfig) -> Option<char> {
    (escape_code == config.open_bracket || escape_code == config.close_bracket)
        .then_some(escape_code)
}

fn make_special_character(escape_code: char) -> Option<char> {
    Some(match escape_code {
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
