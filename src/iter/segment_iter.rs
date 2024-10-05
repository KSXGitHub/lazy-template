use crate::Parse;

#[derive(Debug, Clone, Copy)]
pub struct SegmentResultIter<'a, Parser> {
    template: &'a str,
    parser: &'a Parser,
}

impl<'a, Parser> SegmentResultIter<'a, Parser> {
    pub(crate) fn new(template: &'a str, parser: &'a Parser) -> Self {
        Self { template, parser }
    }
}

impl<'a, Parser> Iterator for SegmentResultIter<'a, Parser>
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
