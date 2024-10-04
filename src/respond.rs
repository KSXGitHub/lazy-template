use core::convert::Infallible;
use derive_more::From;

/// Represent the ability of a responder to response to query from template segments.
pub trait Respond<Query, Output, Error> {
    fn respond(&mut self, query: Query) -> Result<Output, Error>;
}

/// Create a responder from a fallible function.
#[derive(Clone, Copy, From)]
pub struct FunctionResponder<F>(pub F);

impl<Function, Query, Output, Error> Respond<Query, Output, Error> for FunctionResponder<Function>
where
    Function: FnMut(Query) -> Result<Output, Error>,
{
    fn respond(&mut self, query: Query) -> Result<Output, Error> {
        (self.0)(query)
    }
}

/// Create a responder from a total function.
#[derive(Clone, Copy, From)]
pub struct InfallibleFunctionResponder<F>(pub F);

impl<Function, Query, Output> Respond<Query, Output, Infallible>
    for InfallibleFunctionResponder<Function>
where
    Function: FnMut(Query) -> Output,
{
    fn respond(&mut self, query: Query) -> Result<Output, Infallible> {
        Ok((self.0)(query))
    }
}
