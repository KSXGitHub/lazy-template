use crate::{iter::SegmentResultIter, Parse, Render};
use core::{convert::Infallible, fmt, marker::PhantomData};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

#[derive(Debug, Clone, Copy)]
pub struct Template<'a, Parser, Query> {
    iter: SegmentResultIter<'a, Parser>,
    _query: PhantomData<Query>, // phantom Query is necessary to enable type inference later on
}

impl<'a, Parser, Query> Template<'a, Parser, Query> {
    pub(crate) fn new(iter: SegmentResultIter<'a, Parser>) -> Self {
        Self {
            iter,
            _query: PhantomData,
        }
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum TemplateApplicationError<ParseError, QueryError, WriteError> {
    Parse(ParseError),
    Query(QueryError),
    Write(WriteError),
}

impl<'a, Parser, Query> Template<'a, Parser, Query>
where
    Parser: Parse<'a>,
{
    #[cfg(feature = "std")]
    pub fn to_string<RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        respond: Respond,
    ) -> Result<String, TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        RenderOutput: fmt::Display,
        Parser::Output: Render<Respond, RenderOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut buf = String::new();
        self.write_to(&mut buf, respond)?;
        Ok(buf)
    }

    pub fn write_to<Output, RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        output: &mut Output,
        respond: Respond,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        Output: fmt::Write,
        RenderOutput: fmt::Display,
        Parser::Output: Render<Respond, RenderOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut write_error = None;

        self.apply(respond, |response| {
            write_error = write!(output, "{response}").err()
        })
        .map_err(|error| match error {
            TemplateApplicationError::Parse(error) => TemplateApplicationError::Parse(error),
            TemplateApplicationError::Query(error) => TemplateApplicationError::Query(error),
            TemplateApplicationError::Write(error) => match error {},
        })?;

        if let Some(error) = write_error {
            return error.pipe(TemplateApplicationError::Write).pipe(Err);
        }

        Ok(())
    }

    fn apply<HandleSegmentOutput, RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        mut respond: Respond,
        mut handle_query_output: HandleSegmentOutput,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, Infallible>>
    where
        HandleSegmentOutput: FnMut(RenderOutput),
        Parser::Output: Render<Respond, RenderOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        for segment in self.iter {
            let () = segment
                .map_err(TemplateApplicationError::Parse)?
                .render(&mut respond)
                .map_err(TemplateApplicationError::Query)?
                .pipe(&mut handle_query_output);
        }

        Ok(())
    }
}
