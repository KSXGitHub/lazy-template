#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    pub open_bracket: char,
    pub close_bracket: char,
}

impl ParserConfig {
    pub fn curly_braces() -> Self {
        ParserConfig {
            open_bracket: '{',
            close_bracket: '}',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComponentParserInput<'a> {
    pub text: &'a str,
    pub config: ParserConfig,
}
