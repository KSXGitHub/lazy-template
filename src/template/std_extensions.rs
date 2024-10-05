#![cfg(feature = "std")]
use super::{Template, TemplateApplicationError};
use crate::Render;
use core::fmt;
use pipe_trait::Pipe;
use std::io;

impl<SegmentResultIter, Query> Template<SegmentResultIter, Query> {
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
    pub fn to_writer<Writer, Segment, ParseError, RenderOutput, QueryOutput, QueryError, Respond>(
        self,
        writer: &mut Writer,
        respond: Respond,
    ) -> Result<(), TemplateApplicationError<ParseError, QueryError, io::Error>>
    where
        Writer: io::Write,
        SegmentResultIter: Iterator<Item = Result<Segment, ParseError>>,
        RenderOutput: fmt::Display,
        Segment: Render<Respond, RenderOutput, QueryError>,
        Respond: FnMut(Query) -> Result<QueryOutput, QueryError>,
    {
        let mut write_error = None;

        self.apply(respond, |response| {
            write_error = write!(writer, "{response}").err()
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
}
