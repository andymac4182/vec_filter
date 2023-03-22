use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, digit1, space0},
    combinator::{map, recognize, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
    Err
};
use crate::{AST, StructProperties, Value};

#[derive(Debug)]
pub enum ParsePropertyFromString {
    ItemNotFound,
}

fn parse_field<P: StructProperties>(input: &str) -> IResult<&str, P> {
    map_res(
        recognize(tuple((alpha1, space0))),
        |s: &str| match P::from_str(s.trim()) {
            Ok(v) => Ok(v),
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

fn parse_ast<P: StructProperties>(input: &str) -> IResult<&str, AST<P>> {
    let (input, ast) = alt((
        map(
            tuple((parse_field, space0, tag("=="), space0, parse_value)),
            |(field, _, _, _, value)| AST::Equals { field, value },
        ),
        map(
            tuple((parse_field, space0, tag("!="), space0, parse_value)),
            |(field, _, _, _, value)| AST::NotEquals { field, value },
        ),
        map(
            tuple((
                parse_field,
                space0,
                tag("in"),
                space0,
                alt((
                    map(parse_vec_string, Value::VecString),
                    map(parse_vec_int, Value::VecInt),
                    map(parse_value, |x| x),
                )),
            )),
            |(field, _, _, _, value)| AST::Contains { field, value },
        ),
        map(
            tuple((
                delimited(tag("("), parse_ast, tag(")")),
                space0,
                tag("&&"),
                space0,
                delimited(tag("("), parse_ast, tag(")")),
            )),
            |(left, _, _, _, right)| AST::And(Box::new(left), Box::new(right)),
        ),
        map(
            tuple((
                delimited(tag("("), parse_ast, tag(")")),
                space0,
                tag("||"),
                space0,
                delimited(tag("("), parse_ast, tag(")")),
            )),
            |(left, _, _, _, right)| AST::Or(Box::new(left), Box::new(right)),
        ),
    ))(input)?;
    Ok((input, ast))
}

pub fn parse_query<P: StructProperties>(input: &str) -> IResult<&str, AST<P>> {
    delimited(space0, parse_ast, space0)(input)
}
