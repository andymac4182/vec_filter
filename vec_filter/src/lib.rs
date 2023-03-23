#[allow(unused_imports)]
#[macro_use]
extern crate vec_filter_derive;
pub use vec_filter_derive::Filterable;

mod ast;
mod parsers;
mod struct_matcher;

pub use crate::struct_matcher::{StructMatcher, StructProperties};
pub use ast::{CompatibilityError, Value, AST};
pub use parsers::{parse_query, ASTValidationError, FieldNotFound};
