use crate::{
    iter::{LazyParseIter, ParsedTemplate},
    Parse, Template,
};
use core::marker::PhantomData;
use pipe_trait::Pipe;

#[derive(Debug, Clone, Copy)]
pub struct TemplateSystem<Parser, Query> {
    parser: Parser,
    _query: PhantomData<Query>, // phantom Query is necessary to enable type inference later on
}

impl<Parser, Query> TemplateSystem<Parser, Query> {
    pub fn new(parser: Parser) -> Self {
        TemplateSystem {
            parser,
            _query: PhantomData,
        }
    }
}

impl<'a, Parser, Query> TemplateSystem<Parser, Query>
where
    Parser: Parse<'a>,
{
    /// Create a [`Template`] from a template string.
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))] fn main() {}
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// use lazy_template::{Template, simple_curly_braces};
    /// let system = simple_curly_braces();
    /// let template: Template<_, _> = system.lazy_parse("{name} is a {age} years old {gender}");
    /// let output = template
    ///     .to_string(|query| match query {
    ///         "name" => Ok("Alice"),
    ///         "age" => Ok("20"),
    ///         "gender" => Ok("girl"),
    ///         _ => Err(format!("Can't answer {query:?}")),
    ///     })
    ///     .unwrap();
    /// assert_eq!(output, "Alice is a 20 years old girl");
    /// # }
    /// ```
    ///
    /// [`Template`] only parses each segment just before it is needed, meaning that even a template with syntax error
    /// can produce a partial output:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))] fn main() {}
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// let template_string = "{name} is a {age} years } old {gender})"; // incorrectly placed closing curly bracket
    /// let mut output = String::new();
    /// let error = lazy_template::simple_curly_braces()
    ///     .lazy_parse(template_string)
    ///     .write_to(&mut output, |query| match query {
    ///         "name" => Ok("Alice"),
    ///         "age" => Ok("20"),
    ///         "gender" => Ok("girl"),
    ///         _ => Err(format!("Can't answer {query:?}")),
    ///     })
    ///     .unwrap_err();
    /// assert_eq!(
    ///     error.to_string(),
    ///     "Fail to parse query: Unexpected token '}'"
    /// );
    /// assert_eq!(output, "Alice is a 20 years "); // output is partially written
    /// # }
    /// ```
    pub fn lazy_parse(&'a self, text: &'a str) -> Template<LazyParseIter<'a, Parser>, Query> {
        LazyParseIter::new(text, &self.parser).pipe(Template::new)
    }

    /// Parse the template string ahead of time.
    ///
    /// The returned parsed template can be used multiple times with different responders to generate different outputs:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))] fn main() {}
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// let system = lazy_template::simple_curly_braces();
    /// let parsed_template = system.eager_parse::<Vec<_>>("Hello, {name}!").unwrap();
    /// let output = parsed_template
    ///     .to_template()
    ///     .to_string(|query| (query == "name").then_some("Alice").ok_or("Invalid query"))
    ///     .unwrap();
    /// assert_eq!(output, "Hello, Alice!");
    /// let output = parsed_template
    ///     .to_template()
    ///     .to_string(|query| (query == "name").then_some("Bob").ok_or("Invalid query"))
    ///     .unwrap();
    /// assert_eq!(output, "Hello, Bob!");
    /// # }
    /// ```
    ///
    /// Unlike [`lazy_parse`](Self::lazy_parse), this function would fail if the template fails to parse (e.g. syntax error):
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))] fn main() {}
    /// # #[cfg(feature = "std")] fn main() {
    /// # use pretty_assertions::assert_eq;
    /// let template_string = "Hello, {name!"; // missing a closing curly bracket
    /// let error = lazy_template::simple_curly_braces()
    ///     .eager_parse::<Vec<_>>(template_string)
    ///     .unwrap_err();
    /// # assert_eq!(
    /// #     error.to_string(),
    /// #     "Fail to parse query: Unexpected end of input",
    /// # );
    /// # }
    /// ```
    pub fn eager_parse<SegmentContainer>(
        &'a self,
        text: &'a str,
    ) -> Result<ParsedTemplate<SegmentContainer, Query>, Parser::Error>
    where
        SegmentContainer: FromIterator<Parser::Output>,
    {
        LazyParseIter::new(text, &self.parser)
            .collect::<Result<SegmentContainer, Parser::Error>>()
            .map(ParsedTemplate::new)
    }
}

/// Convert a [parser](Parse) into a [`TemplateSystem`].
pub trait IntoTemplateSystem: Sized {
    fn into_template_system<Query>(self) -> TemplateSystem<Self, Query> {
        TemplateSystem::new(self)
    }
}
impl<'a, Parser> IntoTemplateSystem for Parser where Parser: Parse<'a> {}
