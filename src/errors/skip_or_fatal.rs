/// Result type of [`IntoSkipOrFatal::into_skip_or_fatal`].
pub enum SkipOrFatal<Skip, Fatal> {
    Skip(Skip),
    Fatal(Fatal),
}

/// Trait of error type of a component parser.
/// It checks whether the error means "skip" or "fatal".
pub trait IntoSkipOrFatal {
    /// "Skip" means that the parent parser may either try to parse the next type of component or error,
    type Skip;
    /// "Fatal" means that the parent parser should bail immediately.
    type Fatal;
    /// Check whether the error returned from a component parser is skip or fatal.
    fn into_skip_or_fatal(self) -> SkipOrFatal<Self::Skip, Self::Fatal>;
}

impl<Skip, Fatal> IntoSkipOrFatal for SkipOrFatal<Skip, Fatal> {
    type Skip = Skip;
    type Fatal = Fatal;
    fn into_skip_or_fatal(self) -> Self {
        self
    }
}

impl<Error> IntoSkipOrFatal for Option<Error> {
    type Skip = ();
    type Fatal = Error;
    fn into_skip_or_fatal(self) -> SkipOrFatal<Self::Skip, Self::Fatal> {
        match self {
            None => SkipOrFatal::Skip(()),
            Some(error) => SkipOrFatal::Fatal(error),
        }
    }
}
