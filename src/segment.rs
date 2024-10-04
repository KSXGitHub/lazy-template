/// Definition of a segment of a string template.
pub trait Segment<Responder, Output, Error> {
    fn query(self, responder: &mut Responder) -> Result<Output, Error>;
}
