use derive_more::Display;
use pipe_trait::Pipe;

#[derive(Debug, Clone, Copy)]
pub enum Segment<Query> {
    Character(char),
    Expression(Query),
}

#[derive(Debug, Display, Clone, Copy)]
pub enum SegmentOutput<Output> {
    Character(char),
    ExpressionResult(Output),
}

impl<Respond, Output, Error, Query> crate::Segment<Respond, SegmentOutput<Output>, Error>
    for Segment<Query>
where
    Respond: FnMut(Query) -> Result<Output, Error>,
{
    fn query(self, respond: &mut Respond) -> Result<SegmentOutput<Output>, Error> {
        Ok(match self {
            Segment::Character(value) => SegmentOutput::Character(value),
            Segment::Expression(query) => respond(query)?.pipe(SegmentOutput::ExpressionResult),
        })
    }
}
