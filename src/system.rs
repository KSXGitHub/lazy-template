use crate::{iter::SegmentResultIter, Parse, Template};
use core::marker::PhantomData;
use pipe_trait::Pipe;

#[derive(Debug, Clone, Copy)]
pub struct TemplateSystem<Parser, Query> {
    parser: Parser,
    _query: PhantomData<Query>, // phantom Query is necessary to enable type inference later on
}

impl<Parser, Query> TemplateSystem<Parser, Query> {
    pub fn new(parser: Parser) -> Self {
        TemplateSystem {
            parser,
            _query: PhantomData,
        }
    }
}

impl<'a, Parser, Query> TemplateSystem<Parser, Query>
where
    Parser: Parse<'a>,
{
    pub fn lazy_parse(&'a self, text: &'a str) -> Template<'a, Parser, Query> {
        SegmentResultIter::new(text, &self.parser).pipe(Template::new)
    }
}
