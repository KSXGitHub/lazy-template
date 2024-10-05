use crate::Render;
use derive_more::Display;
use pipe_trait::Pipe;

/// Represent a segment of a parsed template.
#[derive(Debug, Clone, Copy)]
pub enum Segment<Query> {
    Character(char),
    Expression(Query),
}

/// Returned upon the [rendering](Render) of a [`Segment`].
///
/// Value of this type can be converted to a string by using the [`Display`] trait.
#[derive(Debug, Display, Clone, Copy)]
pub enum SegmentDisplay<Output> {
    Character(char),
    ExpressionResult(Output),
}

impl<Respond, Output, Error, Query> Render<Respond, SegmentDisplay<Output>, Error>
    for Segment<Query>
where
    Respond: FnMut(Query) -> Result<Output, Error>,
{
    fn render(self, respond: &mut Respond) -> Result<SegmentDisplay<Output>, Error> {
        Ok(match self {
            Segment::Character(value) => SegmentDisplay::Character(value),
            Segment::Expression(query) => respond(query)?.pipe(SegmentDisplay::ExpressionResult),
        })
    }
}
