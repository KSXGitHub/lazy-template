use core::convert::Infallible;

pub fn infallible<Respond, Query, Output>(
    respond: &mut Respond,
) -> impl FnMut(Query) -> Result<Output, Infallible> + '_
where
    Respond: FnMut(Query) -> Output + ?Sized,
{
    |query| Ok(respond(query))
}
