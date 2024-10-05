#![cfg_attr(not(feature = "std"), no_std)]

pub mod utils;

pub mod enclosed;
pub use enclosed::EnclosedTemplateParser;

mod errors;
mod parse;
mod segment;
mod shorthands;
mod system;

pub use errors::*;
pub use parse::*;
pub use segment::*;
pub use shorthands::*;
pub use system::*;
