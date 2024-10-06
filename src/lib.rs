//! # Description
//!
//! This is a string template crate.
//! Instead of requiring a complete set of inputs (such as via a `struct`, a `HashMap`, or a JSON object) to be available,
//! the templates from this crate would send queries (which would usually be the names of the variables) to a function
//! (called "responder") to get the value of each query.
//!
//! # Usage Examples
//!
//! **Example 1:** Lazily parse template
//!
//! ```
//! # #[cfg(not(feature = "std"))] fn main() {}
//! # #[cfg(feature = "std")] fn main() {
//! # use pretty_assertions::assert_eq;
//! let system = lazy_template::simple_curly_braces();
//! let template = system.lazy_parse("{name} is a {age} years old {gender}");
//! let alice_info = template
//!     .to_string(|query| match query {
//!         "name" => Ok("Alice"),
//!         "age" => Ok("20"),
//!         "gender" => Ok("girl"),
//!         _ => Err(format!("Can't answer {query}")),
//!     })
//!     .unwrap();
//! let bob_info = template
//!     .to_string(|query| match query {
//!         "name" => Ok("Bob"),
//!         "age" => Ok("32"),
//!         "gender" => Ok("man"),
//!         _ => Err(format!("Can't answer {query}")),
//!     })
//!     .unwrap();
//! assert_eq!(alice_info, "Alice is a 20 years old girl");
//! assert_eq!(bob_info, "Bob is a 32 years old man");
//! # }
//! ```
//!
//! _see more:_ [`simple_curly_braces`], [`lazy_parse`](crate::TemplateSystem::lazy_parse).
//!
//! **Example 2:** Eagerly parse template:
//!
//! ```
//! # #[cfg(not(feature = "std"))] fn main() {}
//! # #[cfg(feature = "std")] fn main() {
//! # use pretty_assertions::assert_eq;
//! let system = lazy_template::simple_curly_braces();
//! let parsed_template = system
//!     .eager_parse::<Vec<_>>("{name} is a {age} years old {gender}")
//!     .unwrap();
//! let alice_info = parsed_template
//!     .to_template()
//!     .to_string(|query| match query {
//!         "name" => Ok("Alice"),
//!         "age" => Ok("20"),
//!         "gender" => Ok("girl"),
//!         _ => Err(format!("Can't answer {query}")),
//!     })
//!     .unwrap();
//! let bob_info = parsed_template
//!     .to_template()
//!     .to_string(|query| match query {
//!         "name" => Ok("Bob"),
//!         "age" => Ok("32"),
//!         "gender" => Ok("man"),
//!         _ => Err(format!("Can't answer {query}")),
//!     })
//!     .unwrap();
//! assert_eq!(alice_info, "Alice is a 20 years old girl");
//! assert_eq!(bob_info, "Bob is a 32 years old man");
//! # }
//! ```
//!
//! _see more:_ [`simple_curly_braces`], [`eager_parse`](crate::TemplateSystem::eager_parse).
//!
#![cfg_attr(not(feature = "std"), no_std)]

pub mod iter;

pub mod enclosed;
pub use enclosed::EnclosedTemplateParser;

mod errors;
mod parse;
mod render;
mod shorthands;
mod system;
mod template;

pub use errors::*;
pub use parse::*;
pub use render::*;
pub use shorthands::*;
pub use system::*;
pub use template::*;

#[cfg(feature = "std")]
#[doc = include_str!("../README.md")]
#[allow(unused)]
mod test_readme_code_snippets {}
