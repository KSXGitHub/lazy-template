use crate::{iter::LazyParseIter, Parse, Template};
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
    pub fn lazy_parse(&'a self, text: &'a str) -> Template<LazyParseIter<'a, Parser>, Query> {
        LazyParseIter::new(text, &self.parser).pipe(Template::new)
    }
}

/// Convert a [parser](Parse) into a [`TemplateSystem`].
pub trait IntoTemplateSystem: Sized {
    fn into_template_system<Query>(self) -> TemplateSystem<Self, Query> {
        TemplateSystem::new(self)
    }
}
impl<'a, Parser> IntoTemplateSystem for Parser where Parser: Parse<'a> {}
