pub mod simple_query;
pub use simple_query::{SimpleQuery, SimpleQueryParser};

pub mod simple_escape;
pub use simple_escape::{SimpleEscape, SimpleEscapeParser};

pub mod simple_template_system;
pub use simple_template_system::SimpleEnclosedTemplateSystem;

mod config;
mod parser;
mod segment;

pub use config::*;
pub use parser::*;
pub use segment::*;
