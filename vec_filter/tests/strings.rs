use vec_filter::{parse_query, Filterable};

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

        let people = vec![alice.clone(), bob.clone(), carol.clone()];

        let ast = parse_query(input).unwrap_or_else(|err| {
            panic!("Failed to parse input '{}': {:?}", input, err);
        });

        let filtered_people: Vec<Person> = ast.1.apply(&people);
        let expected_people: Vec<Person> = expected_indices
            .iter()
            .map(|index| people[*index].clone())
            .collect();

        assert_eq!(filtered_people, expected_people);
    }
}
