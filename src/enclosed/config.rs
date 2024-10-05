#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    pub open_bracket: char,
    pub close_bracket: char,
    pub escape: Option<char>,
}

impl ParserConfig {
    pub fn curly_braces() -> Self {
        ParserConfig {
            open_bracket: '{',
            close_bracket: '}',
            escape: None,
        }
    }

    pub fn with_escape(mut self, escape: char) -> Self {
        self.escape = Some(escape);
        self
    }

    pub fn without_escape(mut self) -> Self {
        self.escape = None;
        self
    }
}

pub struct ComponentParserInput<'a> {
    pub text: &'a str,
    pub config: ParserConfig,
}
