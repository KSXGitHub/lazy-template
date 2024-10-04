#![cfg_attr(not(feature = "std"), no_std)]

pub mod utils;

pub mod enclosed;
pub use enclosed::EnclosedTemplateParser;

mod segment;
mod shorthands;
mod syntax;
mod system;

pub use segment::*;
pub use shorthands::*;
pub use syntax::*;
pub use system::*;
