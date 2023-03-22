
# Vec Filter

Vec Filter is a Rust library to filter a vector of structs based on a query string. It allows you to specify filter conditions on struct fields with a simple query syntax. The library provides a custom derive macro, `Filterable`, to make your structs filterable with ease.

## Table of Contents

-   [Installation](https://chat.openai.com/chat?model=gpt-4#installation)
-   [Usage](https://chat.openai.com/chat?model=gpt-4#usage)
    -   [Deriving Filterable](https://chat.openai.com/chat?model=gpt-4#deriving-filterable)
    -   [Query Syntax](https://chat.openai.com/chat?model=gpt-4#query-syntax)
    -   [Example](https://chat.openai.com/chat?model=gpt-4#example)
-   [License](https://chat.openai.com/chat?model=gpt-4#license)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
vec_filter = "0.1.0"
```

## Usage

### Deriving Filterable

To make your struct filterable, derive the `Filterable` trait:

```rust
use vec_filter::Filterable;

#[derive(Debug, Clone, Filterable)]
struct Person {
    name: String,
    age: u32,
}
```

### Query Syntax

Vec Filter supports the following operators:

-   `\==`: Equals
-   `!=`: Not equals
-   `in`: Contains (for strings and vectors of strings)

You can also use logical operators `&&` (AND) and `||` (OR) to combine conditions.

### Example

Let's see how to filter a vector of `Person` structs using Vec Filter.

rustCopy code

```rust
use vec_filter::{Filterable, AST};
use vec_filter::Value;

#[derive(Debug, Clone, PartialEq, Filterable)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 40 },
        Person { name: "Charlie".to_string(), age: 50 },
    ];

    let ast = AST::And(
        Box::new(AST::Equals {
            field: "name",
            value: Value::String("Alice".to_string()),
        }),
        Box::new(AST::NotEquals {
            field: "age",
            value: Value::Int(30),
        }),
    );

    let filtered_people = ast.apply(&people);
    println!("{:?}", filtered_people); // []
}
```

In this example, we want to find all people with the name "Alice" and age not equal to 30. The query is represented using an `AST` value. The `apply` method is then called with the `people` vector, and the filtered result is printed.

## License

This project is licensed under the Apache 2.0 License.