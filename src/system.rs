use crate::{iter::SegmentResultIter, Parse, Segment};
use core::{convert::Infallible, fmt, marker::PhantomData};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

pub struct TemplateSystem<Parser, Query> {
    parser: Parser,
    _phantom: PhantomData<Query>,
}

impl<Parser, Query> TemplateSystem<Parser, Query> {
    pub fn new(parser: Parser) -> Self {
        TemplateSystem {
            parser,
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Display, Error, Clone, Copy)]
pub enum TemplateApplicationError<ParseError, QueryError, WriteError> {
    Parse(ParseError),
    Query(QueryError),
    Write(WriteError),
}

impl<'a, Parser, Query> TemplateSystem<Parser, Query>
where
    Parser: Parse<'a>,
{
    #[cfg(feature = "std")]
    pub fn to_string<SegmentOutput, QueryOutput, QueryError, Respond>(
        &'a self,
        template: &'a str,
        respond: Respond,
    ) -> Result<String, TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        SegmentOutput: fmt::Display,
        Parser::Output: Segment<Respond, SegmentOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut buf = String::new();
        self.write_to(&mut buf, template, respond)?;
        Ok(buf)
    }

    pub fn write_to<Output, SegmentOutput, QueryOutput, QueryError, Respond>(
        &'a self,
        output: &mut Output,
        template: &'a str,
        respond: Respond,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        Output: fmt::Write,
        SegmentOutput: fmt::Display,
        Parser::Output: Segment<Respond, SegmentOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut write_error = None;

        self.apply_template(template, respond, |response| {
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

    pub fn segments(&'a self, template: &'a str) -> SegmentResultIter<'a, Parser> {
        SegmentResultIter::new(template, &self.parser)
    }

    fn apply_template<HandleSegmentOutput, SegmentOutput, QueryOutput, QueryError, Respond>(
        &'a self,
        template: &'a str,
        mut respond: Respond,
        mut handle_query_output: HandleSegmentOutput,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, Infallible>>
    where
        HandleSegmentOutput: FnMut(SegmentOutput),
        Parser::Output: Segment<Respond, SegmentOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        for segment in self.segments(template) {
            let () = segment
                .map_err(TemplateApplicationError::Parse)?
                .query(&mut respond)
                .map_err(TemplateApplicationError::Query)?
                .pipe(&mut handle_query_output);
        }

        Ok(())
    }
}
