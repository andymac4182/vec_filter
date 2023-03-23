use crate::{Value, AST};
use core::fmt::Debug;
use core::fmt::Display;
use regex::Regex;
use std::str::FromStr;

pub trait StructProperties: FromStr + Sized + Debug + Display + Clone {
    fn valid_fields() -> Vec<&'static str>;
    fn get_value_type(&self) -> Value;
    fn _check_enum(_: core::marker::PhantomData<Self>) {}
}

pub trait StructMatcher<P>: Sized {
    fn get_property_value(&self, property: &P) -> Option<Value>;

    fn matches_ast(&self, ast: &AST<P>) -> bool {
        match ast {
            AST::Equals { field: _, value: _ }
            | AST::NotEquals { field: _, value: _ }
            | AST::GreaterThan { field: _, value: _ }
            | AST::LessThan { field: _, value: _ }
            | AST::GreaterThanOrEqual { field: _, value: _ }
            | AST::LessThanOrEqual { field: _, value: _ } => self.internal_matches_ast(ast),
            AST::Contains { field, value } => self.matches_contains(field, value),
            AST::StartsWith { field, value } => self.starts_with(field, value),
            AST::EndsWith { field, value } => self.ends_with(field, value),
            AST::RegexMatch { field, value } => self.regex_match(field, value),
            AST::In { field, value } => self.matches_in(field, value),
            AST::And(_, _) | AST::Or(_, _) => self.matches_and_or(ast),
            AST::Not(expr) => !self.matches_ast(expr),
            AST::InvalidField { field_name: _ } => unimplemented!("This should never be called"),
        }
    }
}

trait StructMatcherExt<P>: StructMatcher<P> {
    fn internal_matches_ast(&self, ast: &AST<P>) -> bool;
    fn matches_contains(&self, field: &P, value: &Value) -> bool;
    fn matches_in(&self, field: &P, value: &Value) -> bool;
    fn matches_and_or(&self, ast: &AST<P>) -> bool;
    fn starts_with(&self, field: &P, value: &Value) -> bool;
    fn ends_with(&self, field: &P, value: &Value) -> bool;
    fn regex_match(&self, field: &P, pattern: &Value) -> bool;
}

impl<T: StructMatcher<P>, P> StructMatcherExt<P> for T {
    fn internal_matches_ast(&self, ast: &AST<P>) -> bool {
        match ast {
            AST::Equals { field, value } => self.get_property_value(field) == Some(value.clone()),
            AST::NotEquals { field, value } => {
                self.get_property_value(field) != Some(value.clone())
            }
            AST::GreaterThan { field, value } => self
                .get_property_value(field)
                .map(|v| v > value.clone())
                .unwrap_or(false),
            AST::LessThan { field, value } => self
                .get_property_value(field)
                .map(|v| v < value.clone())
                .unwrap_or(false),
            AST::GreaterThanOrEqual { field, value } => self
                .get_property_value(field)
                .map(|v| v >= value.clone())
                .unwrap_or(false),
            AST::LessThanOrEqual { field, value } => self
                .get_property_value(field)
                .map(|v| v <= value.clone())
                .unwrap_or(false),
            _ => false,
        }
    }
    fn matches_in(&self, field: &P, value: &Value) -> bool {
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

    fn starts_with(&self, field: &P, value: &Value) -> bool {
        match (self.get_property_value(field), value) {
            (Some(Value::String(ref s)), Value::String(ref prefix)) => s.starts_with(prefix),
            (Some(Value::VecString(ref v)), Value::String(ref prefix)) => {
                v.iter().any(|s| s.starts_with(prefix))
            }
            _ => false,
        }
    }

    fn ends_with(&self, field: &P, value: &Value) -> bool {
        match (self.get_property_value(field), value) {
            (Some(Value::String(ref s)), Value::String(ref suffix)) => s.ends_with(suffix),
            (Some(Value::VecString(ref v)), Value::String(ref suffix)) => {
                v.iter().any(|s| s.ends_with(suffix))
            }
            _ => false,
        }
    }

    fn matches_contains(&self, field: &P, value: &Value) -> bool {
        let value = match value {
            Value::String(wrapped_value) => wrapped_value,
            _ => unimplemented!(),
        };

        match self.get_property_value(field) {
            Some(Value::String(ref s)) => {
                s.contains(value)
            }
            Some(Value::VecString(ref v)) => {
                v.iter().any(|s| s.contains(value))
            }
            _ => false,
        }
    }

    fn regex_match(&self, field: &P, pattern_value: &Value) -> bool {
        let pattern = match pattern_value {
            Value::String(pattern) => pattern,
            _ => unimplemented!(),
        };

        match self.get_property_value(field) {
            Some(Value::String(ref s)) => {
                let regex = match Regex::new(pattern) {
                    Ok(regex) => regex,
                    Err(_) => return false,
                };
                regex.is_match(s)
            }
            Some(Value::VecString(ref v)) => {
                let regex = match Regex::new(pattern) {
                    Ok(regex) => regex,
                    Err(_) => return false,
                };
                v.iter().any(|s| regex.is_match(s))
            }
            _ => false,
        }
    }
}
