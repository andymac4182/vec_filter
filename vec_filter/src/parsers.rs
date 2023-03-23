use crate::ast::{is_compatible, CompatibilityError};
use crate::{StructProperties, Value, AST};
use core::fmt::Debug;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, digit1, space0},
    combinator::{map, map_res, recognize},
    multi::{many0, separated_list1},
    sequence::{delimited, tuple},
    Err, IResult,
};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct FieldNotFound {
    field: String,
}

impl FieldNotFound {
    pub fn new(field: &str) -> Self {
        FieldNotFound {
            field: field.to_string(),
        }
    }
}

impl fmt::Display for FieldNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field not found: {}", self.field)
    }
}

impl Error for FieldNotFound {}

enum ParseFieldResult<P> {
    FoundField { field: P },
    InvalidField { field_name: String },
}

fn parse_field_ast<P: StructProperties>(input: &str) -> IResult<&str, ParseFieldResult<P>> {
    map_res(
        recognize(tuple((alphanumeric1, space0))),
        |s: &str| match P::from_str(s.trim()) {
            Ok(v) => Ok(ParseFieldResult::FoundField { field: v }),
            Err(_) => Ok(ParseFieldResult::InvalidField {
                field_name: s.trim().to_string(),
            }),
            #[allow(unreachable_patterns)]
            _ => Err(Err::Error((s, nom::error::ErrorKind::Alpha))),
        },
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, value) = alt((
        map(digit1, |s: &str| Value::Int(s.parse::<i32>().unwrap())),
        map(
            delimited(tag("\""), take_while1(|c: char| c != '"'), tag("\"")),
            |s: &str| Value::String(s.to_string()),
        ),
    ))(input)?;
    Ok((input, value))
}

fn parse_vec_string(input: &str) -> IResult<&str, Vec<String>> {
    let (input, values) =
        delimited(tag("["), separated_list1(tag(","), parse_value), tag("]"))(input)?;
    let values = values
        .into_iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s),
            _ => None,
        })
        .collect();
    Ok((input, values))
}

fn parse_vec_int(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, values) =
        delimited(tag("["), separated_list1(tag(","), parse_value), tag("]"))(input)?;
    let values = values
        .into_iter()
        .filter_map(|v| match v {
            Value::Int(i) => Some(i),
            _ => None,
        })
        .collect();
    Ok((input, values))
}

// Add a new AST validation error type
#[derive(Debug, PartialEq)]
pub enum ASTValidationError<P: StructProperties> {
    InvalidSyntax,
    InvalidField { field_name: String },
    CompatibilityError(CompatibilityError<P>),
}

impl<P: StructProperties> std::fmt::Display for ASTValidationError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ASTValidationError::InvalidSyntax => write!(f, "Invalid syntax"),
            ASTValidationError::InvalidField { field_name } => {
                write!(f, "Invalid field {}", field_name)
            }
            ASTValidationError::CompatibilityError(e) => write!(f, "{}", e),
        }
    }
}

impl<P: StructProperties> std::error::Error for ASTValidationError<P> {}

impl<P: StructProperties> From<CompatibilityError<P>> for ASTValidationError<P> {
    fn from(error: CompatibilityError<P>) -> Self {
        ASTValidationError::CompatibilityError(error)
    }
}

pub fn parse_query<P: StructProperties>(input: &str) -> Result<AST<P>, Vec<ASTValidationError<P>>>
where
    <P as FromStr>::Err: Debug,
{
    // Parse input into a raw AST
    let raw_ast = delimited(space0, parse_raw_ast, space0)(input)
        .map_err(|_| vec![ASTValidationError::InvalidSyntax])?;

    // Validate AST recursively
    fn validate_ast<P: StructProperties>(ast: &AST<P>, errors: &mut Vec<ASTValidationError<P>>)
    where
        <P as FromStr>::Err: Debug,
    {
        match ast {
            AST::And(left, right) | AST::Or(left, right) => {
                validate_ast(left, errors);
                validate_ast(right, errors);
            },
            AST::Not(expr) => validate_ast(expr, errors),
            AST::Equals { field: _, value }
            | AST::NotEquals { field: _, value }
            | AST::In { field: _, value }
            | AST::GreaterThan { field: _, value }
            | AST::LessThan { field: _, value }
            | AST::GreaterThanOrEqual { field: _, value }
            | AST::LessThanOrEqual { field: _, value }
            | AST::Contains { field: _, value }
            | AST::StartsWith { field: _, value }
            | AST::EndsWith { field: _, value }
            | AST::RegexMatch { field: _, value } => {
                if let Err(e) = is_compatible(ast, &Value::wrap(value.clone())) {
                    errors.push(e.into());
                }
            }
            AST::InvalidField { field_name } => {
                errors.push(ASTValidationError::InvalidField {
                    field_name: field_name.to_string(),
                });
            }
        }
    }

    // Perform validation checks
    let mut errors = Vec::new();
    validate_ast(&raw_ast.1, &mut errors);

    let remaining_input = raw_ast.0.trim();
    if !remaining_input.is_empty() && errors.is_empty() {
        errors.push(ASTValidationError::InvalidSyntax);
    }

    if errors.is_empty() {
        // Return the parsed and validated AST
        Ok(raw_ast.1)
    } else {
        // Return all the errors
        Err(errors)
    }
}

fn parse_raw_ast<P: StructProperties>(input: &str) -> IResult<&str, AST<P>> {
    let (input, ast) = alt((
        map(
            tuple((
                parse_field_ast,
                space0,
                alt((
                    tag("=="),
                    tag("!="),
                    tag(">="),
                    tag(">"),
                    tag("<="),
                    tag("<"),
                    tag("contains"),
                    tag("startswith"),
                    tag("endswith"),
                    tag("regexmatch"),
                    tag("contains"),
                )),
                space0,
                parse_value,
            )),
            |(field_ast, _, op, _, value)| match (op, field_ast) {
                (operator, ParseFieldResult::FoundField { field }) => match operator {
                    "==" => AST::Equals { field, value },
                    "!=" => AST::NotEquals { field, value },
                    ">" => AST::GreaterThan { field, value },
                    ">=" => AST::GreaterThanOrEqual { field, value },
                    "<" => AST::LessThan { field, value },
                    "<=" => AST::LessThanOrEqual { field, value },
                    "contains" => AST::Contains { field, value },
                    "startswith" => AST::StartsWith { field, value },
                    "endswith" => AST::EndsWith { field, value },
                    "regexmatch" => AST::RegexMatch { field, value },
                    _ => unreachable!(),
                },
                (_, ParseFieldResult::InvalidField { field_name }) => AST::InvalidField { field_name },
            },
        ),
        map(
            tuple((
                parse_field_ast,
                space0,
                tag("in"),
                space0,
                alt((
                    map(parse_vec_string, Value::VecString),
                    map(parse_vec_int, Value::VecInt),
                    map(parse_value, |x| x),
                )),
            )),
            |(field_ast, _, _, _, value)| match field_ast {
                ParseFieldResult::FoundField { field } => AST::In { field, value },
                ParseFieldResult::InvalidField { field_name } => AST::InvalidField { field_name },
            },
        ),
        map(
            tuple((
                parse_brackets,
                many0(tuple((
                    space0,
                    alt((tag("&&"), tag("||"))),
                    space0,
                    parse_brackets,
                ))),
            )),
            |(first, rest)| {
                rest.into_iter()
                    .fold(first, |acc, (_, op, _, expr)| match op {
                        "&&" => AST::And(Box::new(acc), Box::new(expr)),
                        "||" => AST::Or(Box::new(acc), Box::new(expr)),
                        _ => unreachable!(),
                    })
            },
        ),
    ))(input)?;
    Ok((input, ast))
}

fn parse_brackets<P: StructProperties>(input: &str) -> IResult<&str, AST<P>> {
    map_res(
        tuple((alt((tag("!("), tag("("))), parse_raw_ast, tag(")"))),
        |(op, ast, _)| {
            match op {
                "!(" => Ok(AST::Not(Box::new(ast))),
                "(" => Ok(ast),
                _ => Err(Err::Error(("", nom::error::ErrorKind::Alpha))), // TODO: Find better errors
            }
        },
    )(input)
}
