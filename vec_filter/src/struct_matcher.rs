use crate::{AST, Value};
use std::str::FromStr;

pub trait StructProperties: FromStr {
    fn valid_fields() -> Vec<&'static str>;
}

pub trait StructMatcher<P>: Sized {
    fn get_property_value(&self, property: &P) -> Option<Value>;


    fn matches_ast(&self, ast: &AST<P>) -> bool {
        match ast {
            AST::Equals { field: _, value: _ }
            | AST::NotEquals { field: _, value: _ }
            | AST::GreaterThan { field: _, value: _ }
            | AST::LessThan { field: _, value: _ }
            | AST::GreaterThanEqualTo { field: _, value: _ }
            | AST::LessThanEqualTo { field: _, value: _ } => {
                self.internal_matches_ast(ast)
            }
            AST::Contains { field, value } => self.matches_contains(field, value),
            AST::And(_, _) | AST::Or(_, _) => self.matches_and_or(ast),
        }
    }
}

trait StructMatcherExt<P>: StructMatcher<P> {
    fn internal_matches_ast(&self, ast: &AST<P>) -> bool;

    fn matches_contains(&self, field: &P, value: &Value) -> bool;

    fn matches_and_or(&self, ast: &AST<P>) -> bool;
}

impl<T: StructMatcher<P>, P> StructMatcherExt<P> for T {
    fn internal_matches_ast(&self, ast: &AST<P>) -> bool {
        match ast {
            AST::Equals { field, value } => self.get_property_value(field) == Some(value.clone()),
            AST::NotEquals { field, value } => {
                self.get_property_value(field) != Some(value.clone())
            }
            AST::GreaterThan { field, value } => {
                self.get_property_value(field)
                    .map(|v| v > value.clone())
                    .unwrap_or(false)
            }
            AST::LessThan { field, value } => {
                self.get_property_value(field)
                    .map(|v| v < value.clone())
                    .unwrap_or(false)
            }
            AST::GreaterThanEqualTo { field, value } => {
                self.get_property_value(field)
                    .map(|v| v >= value.clone())
                    .unwrap_or(false)
            }
            AST::LessThanEqualTo { field, value } => {
                self.get_property_value(field)
                    .map(|v| v <= value.clone())
                    .unwrap_or(false)
            }
            _ => false,
        }
    }
    fn matches_contains(&self, field: &P, value: &Value) -> bool {
        match (self.get_property_value(field), value) {
            (Some(Value::String(ref s)), Value::String(ref sub)) => s.contains(sub),
            (Some(Value::VecString(ref v)), Value::String(ref sub)) => {
                v.iter().any(|s| s.contains(sub))
            }
            (Some(Value::String(ref s)), Value::VecString(ref v)) => {
                v.iter().any(|sub| s.contains(sub))
            }
            (Some(Value::VecString(ref v1)), Value::VecString(ref v2)) => {
                v1.iter().any(|s| v2.iter().any(|sub| s.contains(sub)))
            }
            _ => false,
        }
    }

    fn matches_and_or(&self, ast: &AST<P>) -> bool {
        match ast {
            AST::And(left, right) => {
                self.matches_ast(left.as_ref()) && self.matches_ast(right.as_ref())
            }
            AST::Or(left, right) => {
                self.matches_ast(left.as_ref()) || self.matches_ast(right.as_ref())
            }
            _ => false,
        }
    }
}
