use crate::Parse;

#[derive(Debug)]
pub struct LazyParseIter<'a, Parser> {
    template: &'a str,
    parser: &'a Parser,
}

impl<'a, Parser> Clone for LazyParseIter<'a, Parser> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, Parser> Copy for LazyParseIter<'a, Parser> {}

impl<'a, Parser> LazyParseIter<'a, Parser> {
    pub(crate) fn new(template: &'a str, parser: &'a Parser) -> Self {
        Self { template, parser }
    }
}

impl<'a, Parser> Iterator for LazyParseIter<'a, Parser>
where
    Parser: Parse<'a>,
{
    type Item = Result<Parser::Output, Parser::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.template.is_empty() {
            return None;
        }

        let (segment, rest) = match self.parser.parse(self.template) {
            Ok(pair) => pair,
            Err(error) => return Some(Err(error)),
        };

        self.template = rest;
        Some(Ok(segment))
    }
}

impl<'a, Parser> IntoIterator for &'a LazyParseIter<'a, Parser>
where
    Parser: Parse<'a>,
{
    type IntoIter = LazyParseIter<'a, Parser>;
    type Item = Result<Parser::Output, Parser::Error>;
    fn into_iter(self) -> Self::IntoIter {
        *self
    }
}
