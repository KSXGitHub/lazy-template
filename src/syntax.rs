type ParseAsComponentResult<'a, Output, Error> =
    Result<Option<(Output, &'a str)>, <Error as ParseComponentError>::Fatal>;

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
        Self::Error: ParseComponentError,
    {
        let parse_result = self
            .parse(input)
            .map_err(ParseComponentError::skip_or_fatal);
        match parse_result {
            Ok(pair) => Ok(Some(pair)),
            Err(SkipOrFatal::Fatal(error)) => Err(error),
            Err(SkipOrFatal::Skip(_)) => Ok(None),
        }
    }
}

/// This type serves both as the common component error type
/// and the return type of [`ParseComponentError::skip_or_fatal`].
pub enum SkipOrFatal<Skip, Fatal> {
    Skip(Skip),
    Fatal(Fatal),
}

/// Trait of error type of a component parser.
pub trait ParseComponentError {
    /// "Skip" means that the parent parser may either try to parse the next type of component or error,
    type Skip;
    /// "Fatal" means that the parent parser should bail immediately.
    type Fatal;
    /// Check whether the error returned from a component parser is skip or fatal.
    fn skip_or_fatal(self) -> SkipOrFatal<Self::Skip, Self::Fatal>;
}

impl<Skip, Fatal> ParseComponentError for SkipOrFatal<Skip, Fatal> {
    type Skip = Skip;
    type Fatal = Fatal;
    fn skip_or_fatal(self) -> Self {
        self
    }
}
