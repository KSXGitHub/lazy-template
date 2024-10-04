/// Parse a segment.
pub trait Parse<'a, Input = &'a str>: Sized {
    type Output;
    type Error;
    fn parse(&'a self, input: Input) -> Result<(Self::Output, &'a str), Self::Error>;
}
