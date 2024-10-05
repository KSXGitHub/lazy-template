use super::{EnclosedTemplateParser, SimpleEscapeParser, SimpleQuery, SimpleQueryParser};
use crate::TemplateSystem;

pub type SimpleEnclosedTemplateSystem<'a> =
    TemplateSystem<EnclosedTemplateParser<SimpleEscapeParser, SimpleQueryParser>, SimpleQuery<'a>>;
