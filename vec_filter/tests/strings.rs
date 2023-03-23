use vec_filter::{parse_query, Filterable, StructProperties, Value, AST};

use std::fmt;
use std::str::FromStr;

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
    #[case::name_equals_alice("name == \"Alice\"", vec![0])]
    #[case::name_not_equals_alice("name != \"Alice\"", vec![1, 2])]
    #[case::age_equals_30("age == 30", vec![0])]
    #[case::age_not_equals_30("age != 30", vec![1, 2])]
    #[case::interests_contains_reading("interests in [\"reading\"]", vec![0])]
    #[case::interests_contains_cooking("interests in [\"cooking\"]", vec![1])]
    #[case::interests_contains_hiking("interests in [\"hiking\"]", vec![0, 2])]
    #[case::name_equals_alice_and_age_equals_30("(name == \"Alice\") && (age == 30)", vec![0])]
    #[case::name_equals_alice_or_name_equals_bob("(name == \"Alice\") || (name == \"Bob\")", vec![0, 1])]
    #[case::name_in_alice_or_bob("name in [\"Alice\",\"Bob\"]", vec![0, 1])]
    #[case::interests_in_hiking("interests in \"hiking\"", vec![0, 2])]
    #[case::name_alice_bob_eve("(name == \"Alice\") || (name == \"Bob\") || (name == \"Eve\")", vec![0, 1])]
    #[case::interests_hiking_age_25("(interests in [\"hiking\"]) && (age == 25)", vec![2])]
    #[case::name_equals_alice_interests_hiking_age_20("((name == \"Alice\") && (interests in [\"hiking\"])) || (age == 20)", vec![0, 1])]
    #[case::interests_hiking_age_20_or_25("(interests in [\"hiking\"]) && ((age == 20) || (age == 25))", vec![2])]
    #[case::age_greater_than_25("age > 25", vec![0])]
    #[case::age_less_than_25("age < 25", vec![1])]
    #[case::age_greater_than_equal_to_25("age >= 25", vec![0, 2])]
    #[case::age_less_than_equal_to_25("age <= 25", vec![1, 2])]
    #[case::name_startswith_al("name startswith \"Al\"", vec![0])]
    #[case::name_endswith_ce("name endswith \"ce\"", vec![0])]
    #[case::name_regexmatch_alice_bob("name regexmatch \"^(Alice|Bob)$\"", vec![0, 1])]
    #[case::not_operation("!(age == 30)", vec![1, 2])]
    fn test_filtering(#[case] input: &str, #[case] expected_indices: Vec<usize>) {
        let alice = Person {
            name: "Alice".to_string(),
            age: 30,
            interests: vec!["reading".to_string(), "hiking".to_string()],
        };
        let bob = Person {
            name: "Bob".to_string(),
            age: 20,
            interests: vec!["swimming".to_string(), "cooking".to_string()],
        };
        let carol = Person {
            name: "Carol".to_string(),
            age: 25,
            interests: vec!["hiking".to_string(), "painting".to_string()],
        };

        let people = vec![alice, bob, carol];

        let ast = parse_query(input).unwrap_or_else(|err| {
            panic!("Failed to parse input '{}': {:?}", input, err);
        });

        let filtered_people: Vec<Person> = ast.apply(&people);
        let expected_people: Vec<Person> = expected_indices
            .iter()
            .map(|index| people[*index].clone())
            .collect();

        assert_eq!(filtered_people, expected_people);
    }

    #[rstest]
    #[case::equals_operation("age == 30", AST::Equals { field: PersonProperties::age, value: Value::Int(30) })]
    #[case::not_equals_operation("age != 30", AST::NotEquals { field: PersonProperties::age, value: Value::Int(30) })]
    #[case::contains_operation("interests in [\"hiking\"]", AST::In { field: PersonProperties::interests, value: Value::VecString(vec!["hiking".to_string()]) })]
    #[case::and_operation("(age == 30) && (interests in [\"hiking\"])", AST::And(
        Box::new(AST::Equals { field: PersonProperties::age, value: Value::Int(30) }),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::VecString(vec!["hiking".to_string()]) })
    ))]
    #[case::or_operation("(age == 30) || (interests in [\"hiking\"])", AST::Or(
        Box::new(AST::Equals { field: PersonProperties::age, value: Value::Int(30) }),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::VecString(vec!["hiking".to_string()]) })
    ))]
    #[case::greater_than_operation("age > 25", AST::GreaterThan { field: PersonProperties::age, value: Value::Int(25) })]
    #[case::greater_than_or_equal_operation("age >= 25", AST::GreaterThanOrEqual { field: PersonProperties::age, value: Value::Int(25) })]
    #[case::less_than_operation("age < 25", AST::LessThan { field: PersonProperties::age, value: Value::Int(25) })]
    #[case::less_than_or_equal_operation("age <= 25", AST::LessThanOrEqual { field: PersonProperties::age, value: Value::Int(25) })]
    #[case::interests_contains_operation("interests in \"hiking\"", AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })]
    #[case::not_operation("!(age == 30)", AST::Not(Box::new(AST::Equals { field: PersonProperties::age, value: Value::Int(30) })))]
    #[case::complex_and_operation("(age > 25) && (interests in \"hiking\")", AST::And(
        Box::new(AST::GreaterThan { field: PersonProperties::age, value: Value::Int(25) }),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
    ))]
    #[case::complex_or_operation("(age < 25) || (interests in \"hiking\")", AST::Or(
        Box::new(AST::LessThan { field: PersonProperties::age, value: Value::Int(25) }),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
    ))]
    #[case::complex_mixed_operations("((age > 20) && (age < 30)) || (interests in \"hiking\")", AST::Or(
        Box::new(AST::And(
            Box::new(AST::GreaterThan { field: PersonProperties::age, value: Value::Int(20) }),
            Box::new(AST::LessThan { field: PersonProperties::age, value: Value::Int(30) })
        )),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
    ))]
    #[case::name_equals_operation("name == \"Alice\"", AST::Equals { field: PersonProperties::name, value: Value::String("Alice".to_string()) })]
    #[case::name_not_equals_operation("name != \"Alice\"", AST::NotEquals { field: PersonProperties::name, value: Value::String("Alice".to_string()) })]
    #[case::name_contains_operation("name contains \"A\"", AST::Contains { field: PersonProperties::name, value: Value::String("A".to_string()) })]
    #[case::and_and_operation("((age > 20) && (age < 30)) && (interests in \"hiking\")", AST::And(
        Box::new(AST::And(
            Box::new(AST::GreaterThan { field: PersonProperties::age, value: Value::Int(20) }),
            Box::new(AST::LessThan { field: PersonProperties::age, value: Value::Int(30) })
        )),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
    ))]
    #[case::or_or_operation("((age > 20) && (age < 30)) || ((name in \"A\") || (interests in \"hiking\"))", AST::Or(
        Box::new(AST::And(
            Box::new(AST::GreaterThan { field: PersonProperties::age, value: Value::Int(20) }),
            Box::new(AST::LessThan { field: PersonProperties::age, value: Value::Int(30) })
        )),
        Box::new(AST::Or(
            Box::new(AST::In { field: PersonProperties::name, value: Value::String("A".to_string()) }),
            Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
        ))
    ))]
    #[case::complex_nested_operations("(!(age > 20) && (name contains \"A\")) || (interests in \"hiking\")", AST::Or(
        Box::new(AST::And(
            Box::new(AST::Not(Box::new(AST::GreaterThan { field: PersonProperties::age, value: Value::Int(20) }))),
            Box::new(AST::Contains { field: PersonProperties::name, value: Value::String("A".to_string()) })
        )),
        Box::new(AST::In { field: PersonProperties::interests, value: Value::String("hiking".to_string()) })
    ))]
    fn test_parse_query_ast_output(#[case] input: &str, #[case] expected_ast: AST<PersonProperties>) {
        let ast = parse_query::<PersonProperties>(input).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[derive(Clone, Debug)]
    struct DummyProperties;

    impl StructProperties for DummyProperties {
        fn valid_fields() -> Vec<&'static str> {
            vec![]
        }

        fn get_value_type(&self) -> Value {
            Value::String("".to_string())
        }
    }

    impl FromStr for DummyProperties {
        type Err = ();

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            Ok(DummyProperties)
        }
    }

    impl fmt::Display for DummyProperties {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[]")
        }
    }
    /*
    #[rstest::rstest]
    #[case("name == 123", "Expected a string for comparison with '=='")]
    #[case("age == \"Alice\"", "Expected a number for comparison with '=='")]
    #[case("interests in 123", "Expected a string or array for the 'in' operator")]
    #[case("name startswith 123", "Expected a string for the 'startswith' operator")]
    #[case("name endswith [\"Alice\"]", "Expected a string for the 'endswith' operator")]
    #[case("name regexmatch 123", "Expected a string for the 'regexmatch' operator")]
    fn test_error_handling(#[case] input: &str, #[case] expected_error: &str) {
            let result = parse_query(input);

            match result {
                Ok(_) => panic!("Expected an error, but got Ok(_)"),
                Err(nom::Err::Error(e)) => {
                    assert_eq!(e.input, expected_error);
                    // You can also assert the error kind if necessary.
                }
                Err(nom::Err::Failure(e)) => panic!("Expected an error, but got Failure: {:?}", e),
                Err(nom::Err::Incomplete(_)) => panic!("Expected an error, but got Incomplete"),
            }
        }
        */
}
