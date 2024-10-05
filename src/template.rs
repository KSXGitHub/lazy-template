use crate::Render;
use core::{convert::Infallible, fmt, marker::PhantomData};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

#[derive(Debug, Clone, Copy)]
pub struct Template<SegmentResultIter, Query> {
    iter: SegmentResultIter,
    _query: PhantomData<Query>, // phantom Query is necessary to enable type inference later on
}

impl<SegmentResultIter, Query> Template<SegmentResultIter, Query> {
    pub(crate) fn new(iter: SegmentResultIter) -> Self {
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

impl<SegmentResultIter, Query> Template<SegmentResultIter, Query> {
    #[cfg(feature = "std")]
    pub fn to_string<Segment, ParseError, RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        respond: Respond,
    ) -> Result<String, TemplateApplicationError<ParseError, QueryError, fmt::Error>>
    where
        SegmentResultIter: Iterator<Item = Result<Segment, ParseError>>,
        RenderOutput: fmt::Display,
        Segment: Render<Respond, RenderOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut buf = String::new();
        self.write_to(&mut buf, respond)?;
        Ok(buf)
    }

    pub fn write_to<Output, Segment, ParseError, RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        output: &mut Output,
        respond: Respond,
    ) -> Result<(), TemplateApplicationError<ParseError, QueryError, fmt::Error>>
    where
        Output: fmt::Write,
        SegmentResultIter: Iterator<Item = Result<Segment, ParseError>>,
        RenderOutput: fmt::Display,
        Segment: Render<Respond, RenderOutput, QueryError>,
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

    fn apply<
        Segment,
        ParseError,
        RenderOutput,
        QueryOutput,
        QueryError,
        Respond,
        HandleSegmentOutput,
    >(
        self,
        mut respond: Respond,
        mut handle_query_output: HandleSegmentOutput,
    ) -> Result<(), TemplateApplicationError<ParseError, QueryError, Infallible>>
    where
        SegmentResultIter: Iterator<Item = Result<Segment, ParseError>>,
        HandleSegmentOutput: FnMut(RenderOutput),
        Segment: Render<Respond, RenderOutput, QueryError>,
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
