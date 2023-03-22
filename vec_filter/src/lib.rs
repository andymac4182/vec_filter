#[allow(unused_imports)]
#[macro_use]
extern crate vec_filter_derive;
pub use vec_filter_derive::Filterable;

mod ast;    
mod struct_matcher;
mod parsers;

pub use ast::{AST, Value};
pub use crate::struct_matcher::{StructMatcher, StructProperties};
pub use parsers::{parse_query, ParsePropertyFromString};