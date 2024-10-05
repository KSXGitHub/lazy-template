pub mod simple_query;
pub use simple_query::{SimpleQuery, SimpleQueryParser};

pub mod simple_escape;
pub use simple_escape::SimpleEscapeParser;

pub mod simple_template;
pub use simple_template::SimpleEnclosedTemplate;

mod config;
mod parser;
mod segment;

pub use config::*;
pub use parser::*;
pub use segment::*;
