use crate::{Parse, Respond, Segment};
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
    pub fn to_string<SegmentOutput, QueryOutput, QueryError, Responder>(
        &'a self,
        template: &'a str,
        responder: Responder,
    ) -> Result<String, TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        SegmentOutput: fmt::Display,
        Parser::Output: Segment<Responder, SegmentOutput, QueryError>,
        Responder: Respond<Query, QueryOutput, QueryError>,
    {
        let mut buf = String::new();
        self.write_to(&mut buf, template, responder)?;
        Ok(buf)
    }

    pub fn write_to<Output, SegmentOutput, QueryOutput, QueryError, Responder>(
        &'a self,
        output: &mut Output,
        template: &'a str,
        responder: Responder,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, fmt::Error>>
    where
        Output: fmt::Write,
        SegmentOutput: fmt::Display,
        Parser::Output: Segment<Responder, SegmentOutput, QueryError>,
        Responder: Respond<Query, QueryOutput, QueryError>,
    {
        let mut write_error = None;

        self.apply_template(template, responder, |response| {
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

    fn apply_template<HandleSegmentOutput, SegmentOutput, QueryOutput, QueryError, Responder>(
        &'a self,
        template: &'a str,
        mut responder: Responder,
        mut handle_query_output: HandleSegmentOutput,
    ) -> Result<(), TemplateApplicationError<Parser::Error, QueryError, Infallible>>
    where
        HandleSegmentOutput: FnMut(SegmentOutput),
        Parser::Output: Segment<Responder, SegmentOutput, QueryError>,
        Responder: Respond<Query, QueryOutput, QueryError>,
    {
        if template.is_empty() {
            return Ok(());
        }

        let (segment, rest) = self
            .parser
            .parse(template)
            .map_err(TemplateApplicationError::Parse)?;

        let () = segment
            .query(&mut responder)
            .map_err(TemplateApplicationError::Query)?
            .pipe(&mut handle_query_output);

        self.apply_template(rest, responder, handle_query_output)
    }
}
