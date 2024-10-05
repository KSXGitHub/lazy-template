/// Represent the ability of a segment of a string template to render template output.
pub trait Render<Respond, Output, Error> {
    /// Optionally send a query the respond function for output, then render.
    fn render(self, respond: &mut Respond) -> Result<Output, Error>;
}
