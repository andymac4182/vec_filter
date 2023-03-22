use std::convert::TryInto;
use crate::StructMatcher;

/// An enumeration of Abstract Syntax Tree (AST) nodes representing various query operations.
/// P: The generic type parameter representing the field type in the AST.
#[derive(Debug, PartialEq, Clone)]
pub enum AST<P> {
    /// Represents an equality operation: field == value.
    Equals { field: P, value: Value },

    /// Represents an inequality operation: field != value.
    NotEquals { field: P, value: Value },

    /// Represents a containment operation: field contains value.
    Contains { field: P, value: Value },

    /// Represents a greater-than operation: field > value.
    GreaterThan { field: P, value: Value },

    /// Represents a less-than operation: field < value.
    LessThan { field: P, value: Value },

    /// Represents a greater-than-or-equal-to operation: field >= value.
    GreaterThanEqualTo { field: P, value: Value },

    /// Represents a less-than-or-equal-to operation: field <= value.
    LessThanEqualTo { field: P, value: Value },

    /// Represents a logical AND operation between two AST nodes.
    And(Box<AST<P>>, Box<AST<P>>),

    /// Represents a logical OR operation between two AST nodes.
    Or(Box<AST<P>>, Box<AST<P>>),
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

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}

impl Into<Value> for u32 {
    fn into(self) -> Value {
        Value::Int(self.try_into().unwrap())
    }
}

impl Into<Value> for i32 {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Into<Value> for Vec<String> {
    fn into(self) -> Value {
        Value::VecString(self)
    }
}

impl Into<Value> for Vec<i32> {
    fn into(self) -> Value {
        Value::VecInt(self)
    }
}