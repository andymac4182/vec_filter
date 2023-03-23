use vec_filter::{parse_query, ASTValidationError, CompatibilityError, Filterable, Value, AST};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[derive(Debug, Clone, PartialEq, Filterable)]
    pub struct Person {
        pub name: String,
        pub age: u32,
        pub interests: Vec<String>,
    }

    #[rstest]
    #[rstest]
    #[case::invalid_syntax("field1 == 'value'", ASTValidationError::InvalidSyntax)]
    #[case::invalid_field("field1 == \"value\"", ASTValidationError::InvalidField { field_name: "field1".to_string() })]
    #[case::invalid_comparison("age == \"Alice\"", ASTValidationError::CompatibilityError(CompatibilityError {
        ast: AST::Equals {
            field: PersonProperties::age,
            value: Value::String("Alice".to_string()),
        },
        field: PersonProperties::age,
        provided_value: Value::String("Alice".to_string()),
        valid_values: vec![Value::Int(i32::default())],
    }))]
    #[case::invalid_and_operator(
        "age > 25 && (name == \"Alice\"",
        ASTValidationError::InvalidSyntax
    )]
    #[case::invalid_or_operator(
        "age > 25 || (name == \"Alice\")",
        ASTValidationError::InvalidSyntax
    )]
    #[case::invalid_parentheses(
        "((age > 25) && (name == \"Alice\"",
        ASTValidationError::InvalidSyntax
    )]
    #[case::invalid_value_type("interests == \"reading\"", ASTValidationError::CompatibilityError(CompatibilityError {
        ast: AST::Equals {
            field: PersonProperties::interests,
            value: Value::String("reading".to_string()),
        },
        field: PersonProperties::interests,
        provided_value: Value::String("reading".to_string()),
        valid_values: vec![Value::VecString(vec![])],
    }))]
    #[case::nested_and_or(
        "age > 25 || (name == \"Alice\" && (interests in [\"reading\"]))",
        ASTValidationError::InvalidSyntax
    )]
    #[case::missing_operator(
        "(name == \"Alice\") (name == \"Bob\")",
        ASTValidationError::InvalidSyntax
    )]
    #[case::incorrect_and("age > 25 & (name == \"Alice\")", ASTValidationError::InvalidSyntax)]
    #[case::incorrect_or("age > 25 | (name == \"Alice\")", ASTValidationError::InvalidSyntax)]
    #[case::unmatched_parentheses(
        "((age > 25) && (name == \"Alice\")",
        ASTValidationError::InvalidSyntax
    )]
    #[case::missing_parentheses("age > 25 && name == \"Alice\"", ASTValidationError::InvalidSyntax)]
    //    #[case::extra_parentheses("((age > 25) && ((name == \"Alice\")))", ASTValidationError::InvalidSyntax)]
    #[case::mix_or_and_no_brackets(
        "(age > 25) && (name == \"Alice\") || age == 3",
        ASTValidationError::InvalidSyntax
    )]
    fn parse_query_error_handling(
        #[case] input: &str,
        #[case] expected_error: ASTValidationError<PersonProperties>,
    ) {
        let result = parse_query::<PersonProperties>(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err()[0], expected_error);
    }
}
