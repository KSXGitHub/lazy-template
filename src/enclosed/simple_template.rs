use super::{EnclosedTemplateParser, SimpleEscapeParser, SimpleQuery, SimpleQueryParser};
use crate::TemplateSystem;

pub type SimpleEnclosedTemplate<'a> =
    TemplateSystem<EnclosedTemplateParser<SimpleEscapeParser, SimpleQueryParser>, SimpleQuery<'a>>;
