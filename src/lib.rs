#![cfg_attr(not(feature = "std"), no_std)]

pub mod enclosed;
pub use enclosed::EnclosedTemplateParser;

mod respond;
mod segment;
mod shorthands;
mod syntax;
mod system;

pub use respond::*;
pub use segment::*;
pub use shorthands::*;
pub use syntax::*;
pub use system::*;
