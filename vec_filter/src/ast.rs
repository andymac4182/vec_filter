use crate::{StructMatcher, StructProperties};
use std::convert::TryInto;
use std::fmt::Debug;
use std::str::FromStr;

/// An enumeration of Abstract Syntax Tree (AST) nodes representing various query operations.
/// P: The generic type parameter representing the field type in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum AST<P> {
    /// Represents an equality operation: field == value.
    Equals {
        field: P,
        value: Value,
    },

    /// Represents an inequality operation: field != value.
    NotEquals {
        field: P,
        value: Value,
    },

    /// Represents a containment operation: field in value.
    In {
        field: P,
        value: Value,
    },

    /// Represents a containment operation: field contains value.
    Contains {
        field: P,
        value: Value,
    },

    /// Represents a greater-than operation: field > value.
    GreaterThan {
        field: P,
        value: Value,
    },

    /// Represents a less-than operation: field < value.
    LessThan {
        field: P,
        value: Value,
    },

    /// Represents a greater-than-or-equal-to operation: field >= value.
    GreaterThanOrEqual {
        field: P,
        value: Value,
    },

    /// Represents a less-than-or-equal-to operation: field <= value.
    LessThanOrEqual {
        field: P,
        value: Value,
    },

    /// Represents a starts-with operation: field starts with value.
    StartsWith {
        field: P,
        value: Value,
    },

    /// Represents an ends-with operation: field ends with value.
    EndsWith {
        field: P,
        value: Value,
    },

    /// Represents a regex-match operation: field matches regex pattern.
    RegexMatch {
        field: P,
        value: Value,
    },

    InvalidField {
        field_name: String,
    },

    /// Represents a logical AND operation between two AST nodes.
    And(Box<AST<P>>, Box<AST<P>>),

    /// Represents a logical OR operation between two AST nodes.
    Or(Box<AST<P>>, Box<AST<P>>),

    Not(Box<AST<P>>),
}

impl<P> AST<P> {
    pub fn apply<F: StructMatcher<P> + Clone>(&self, items: &[F]) -> Vec<F> {
        items
            .iter()
            .filter(|item| item.matches_ast(self))
            .cloned()
            .collect()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    VecString(Vec<String>),
    VecInt(Vec<i32>),
}

impl Value {
    pub fn wrap<T>(t: T) -> Self
    where
        T: Into<Value>,
    {
        t.into()
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        self
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}

impl From<u32> for Value {
    fn from(val: u32) -> Self {
        Value::Int(val.try_into().unwrap())
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Int(val)
    }
}

impl From<Vec<String>> for Value {
    fn from(val: Vec<String>) -> Self {
        Value::VecString(val)
    }
}

impl From<Vec<i32>> for Value {
    fn from(val: Vec<i32>) -> Self {
        Value::VecInt(val)
    }
}

fn valid_comparison_values<P>(ast: &AST<P>, value: &Value) -> Vec<Value> {
    match ast {
        AST::Equals { field: _, value: _ } => {
            vec![value.clone()]
        }
        AST::NotEquals { field: _, value: _ } => {
            vec![value.clone()]
        }
        AST::GreaterThan { field: _, value: _ } => {
            if let Value::Int(ref n) = value {
                vec![Value::Int(*n - 1)]
            } else {
                vec![]
            }
        }
        AST::LessThan { field: _, value: _ } => {
            if let Value::Int(ref n) = value {
                vec![Value::Int(*n + 1)]
            } else {
                vec![]
            }
        }
        AST::GreaterThanOrEqual { field: _, value: _ } => {
            if let Value::Int(ref n) = value {
                vec![value.clone(), Value::Int(*n - 1)]
            } else {
                vec![]
            }
        }
        AST::LessThanOrEqual { field: _, value: _ } => {
            if let Value::Int(ref n) = value {
                vec![value.clone(), Value::Int(*n + 1)]
            } else {
                vec![]
            }
        }
        AST::Contains { field: _, value: _ } => {
            if let Value::String(ref s) = value {
                vec![Value::String(s.clone())]
            } else {
                vec![]
            }
        }
        AST::StartsWith { field: _, value: _ } => {
            if let Value::String(ref s) = value {
                vec![Value::String(s.clone())]
            } else {
                vec![]
            }
        }
        AST::EndsWith { field: _, value: _ } => {
            if let Value::String(ref s) = value {
                vec![Value::String(s.clone())]
            } else {
                vec![]
            }
        }
        AST::RegexMatch { field: _, value: _ } => {
            if let Value::String(ref s) = value {
                vec![Value::String(s.clone())]
            } else {
                vec![]
            }
        }
        AST::In { field: _, value: _ } => match value {
            Value::String(_) | Value::VecString(_) => {
                vec![Value::String(String::default()), Value::VecString(vec![])]
            }
            Value::Int(_) | Value::VecInt(_) => {
                vec![Value::Int(i32::default()), Value::VecInt(vec![])]
            }
        },
        AST::And(left, right) => {
            let mut values = vec![];
            values.append(&mut valid_comparison_values(left, value));
            values.append(&mut valid_comparison_values(right, value));
            values
        }
        AST::Or(left, right) => {
            let mut values = vec![];
            values.append(&mut valid_comparison_values(left, value));
            values.append(&mut valid_comparison_values(right, value));
            values
        }
        AST::InvalidField { field_name: _ } | AST::Not { .. } => unimplemented!("This should never be called"),
    }
}

#[derive(Debug, PartialEq)]
pub struct CompatibilityError<P: StructProperties> {
    pub ast: AST<P>,
    pub field: P,
    pub provided_value: Value,
    pub valid_values: Vec<Value>,
}

impl<P: StructProperties> std::fmt::Display for CompatibilityError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Incompatible value for field {:?}. Operation: {:?}, Provided value: {:?}, valid value options: {:?}",
            self.field, self.ast, self.provided_value, self.valid_values
        )
    }
}

impl<P: StructProperties> std::error::Error for CompatibilityError<P> {}

pub fn is_compatible<P: StructProperties>(
    ast: &AST<P>,
    parsed_value: &Value,
) -> Result<(), CompatibilityError<P>>
where
    <P as FromStr>::Err: Debug,
{
    fn variants_match(a: &Value, b: &Value) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    let (field, field_value_type) = match ast {
        AST::Equals { field, .. }
        | AST::NotEquals { field, .. }
        | AST::In { field, .. }
        | AST::GreaterThan { field, .. }
        | AST::LessThan { field, .. }
        | AST::GreaterThanOrEqual { field, .. }
        | AST::LessThanOrEqual { field, .. }
        | AST::StartsWith { field, .. }
        | AST::EndsWith { field, .. }
        | AST::RegexMatch { field, .. }
        | AST::Contains { field, .. } 
            => (field, field.get_value_type()),
        AST::And { .. }
        | AST::Or { .. }
        | AST::InvalidField { .. } 
        | AST::Not { .. }
            => unreachable!("This variant should not be handled"),
    };

    let valid_values = valid_comparison_values(ast, &field_value_type);

    if valid_values
        .iter()
        .any(|value| variants_match(value, parsed_value))
    {
        Ok(())
    } else {
        Err(CompatibilityError {
            ast: ast.clone(),
            field: field.clone(),
            provided_value: parsed_value.clone(),
            valid_values,
        })
    }
}
