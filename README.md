
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

-   `==`: Equals
-   `!=`: Not equals
-   `>`: Greater Than
-   `>=`: Greater than or equal to
-   `<`: Less than
-   `<=`: Less than or equal to
-   `in`: Contains (for strings and vectors of strings)

You can also use logical operators `&&` (AND) and `||` (OR) to combine conditions. 

#### Parentheses
To set the precedence of the operations, use parentheses. Expressions enclosed in parentheses will be evaluated first. If you want to group conditions, you can use parentheses to create more complex queries.

For example:

* `((age >= 25) && (interests in ["hiking"])) || (name == "Alice")`: This query will match items where the age is greater than or equal to 25 and the interests contain "hiking", or the name is equal to "Alice".
* `(name == "Alice") || ((age < 30) && (interests in ["cooking"]))`: This query will match items where the name is equal to "Alice", or the age is less than 30 and the interests contain "cooking".

By using parentheses, you can build complex queries that combine multiple conditions with different levels of precedence to achieve precise filtering.

#### Query Examples
* `name == "John Doe"`
* `age != 30`
* `age == 25 && name != "Alice"`
* `role in ["admin", "manager"]`
* `salary > 50000 || role == "developer"`
* `(name == "Alice" || name == "Bob") && age > 20`
* `age < 18 && (role == "intern" || role == "student")`
* `email == "johndoe@example.com" || (role == "manager" && department == "HR")`
* `age >= 30 && (role in ["team_lead", "manager"] || department != "IT")`
* `((name == "Alice" || name == "Bob") && department == "Sales") || (age > 35 && role == "manager")`

### Example

Let's see how to filter a vector of `Person` structs using Vec Filter.

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