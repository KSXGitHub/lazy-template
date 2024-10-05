#![cfg_attr(not(feature = "std"), no_std)]

pub mod iter;
pub mod utils;

pub mod enclosed;
pub use enclosed::EnclosedTemplateParser;

mod errors;
mod parse;
mod render;
mod shorthands;
mod system;

pub use errors::*;
pub use parse::*;
pub use render::*;
pub use shorthands::*;
pub use system::*;
