use std::convert::TryInto;
use crate::vec_filter;

#[derive(Debug, PartialEq, Clone)]
pub enum AST<P> {
    Equals { field: P, value: Value },
    NotEquals { field: P, value: Value },
    Contains { field: P, value: Value },
    And(Box<AST<P>>, Box<AST<P>>),
    Or(Box<AST<P>>, Box<AST<P>>),
}

impl<P> AST<P> {
    pub fn apply<F: vec_filter<P> + Clone>(&self, items: &[F]) -> Vec<F> {
        items
            .iter()
            .filter(|item| item.matches_ast(self))
            .cloned()
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
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