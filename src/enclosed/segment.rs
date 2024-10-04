use crate::Respond;
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

impl<Responder, Output, Error, Query> crate::Segment<Responder, SegmentOutput<Output>, Error>
    for Segment<Query>
where
    Responder: Respond<Query, Output, Error>,
{
    fn query(self, responder: &mut Responder) -> Result<SegmentOutput<Output>, Error> {
        Ok(match self {
            Segment::Character(value) => SegmentOutput::Character(value),
            Segment::Expression(query) => responder
                .respond(query)?
                .pipe(SegmentOutput::ExpressionResult),
        })
    }
}
