use crate::{IntoSkipOrFatal, SkipOrFatal};

type ParseAsComponentResult<'a, Output, Error> =
    Result<Option<(Output, &'a str)>, <Error as IntoSkipOrFatal>::Fatal>;

/// Parse a segment.
pub trait Parse<'a, Input = &'a str>: Sized {
    type Output;
    type Error;
    fn parse(&'a self, input: Input) -> Result<(Self::Output, &'a str), Self::Error>;

    fn parse_as_component(
        &'a self,
        input: Input,
    ) -> ParseAsComponentResult<'a, Self::Output, Self::Error>
    where
        Self::Error: IntoSkipOrFatal,
    {
        let parse_result = self
            .parse(input)
            .map_err(IntoSkipOrFatal::into_skip_or_fatal);
        match parse_result {
            Ok(pair) => Ok(Some(pair)),
            Err(SkipOrFatal::Fatal(error)) => Err(error),
            Err(SkipOrFatal::Skip(_)) => Ok(None),
        }
    }
}
