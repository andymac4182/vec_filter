use vec_filter::{parse_query, Filterable};

#[derive(Debug, Clone, PartialEq, Filterable)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub interests: Vec<String>,
}

fn main() {
    let input = "(age > 25) && (name == \"Alice\") || age == 3";
    let result = parse_query(input);
    println!("AST: {:?}", result.as_ref().unwrap());

    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            interests: vec!["reading".to_string(), "hiking".to_string()],
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            interests: vec!["swimming".to_string(), "cooking".to_string()],
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
            interests: vec!["hiking".to_string(), "painting".to_string()],
        },
    ];

    match result {
        Ok(ast) => {
            let filtered_people: Vec<Person> = ast.apply(&people);
            println!("Filtered people: {:?}", filtered_people);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
