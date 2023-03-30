
# Vec Filter

Vec Filter is a Rust library to filter a vector of structs based on a query string. It allows you to specify filter conditions on struct fields with a simple query syntax. The library provides a custom derive macro, `Filterable`, to make your structs filterable with ease.

## Table of Contents

-   [Installation](#installation)
-   [Usage](#usage)
    -   [Deriving Filterable](#deriving-filterable)
    -   [Query Syntax](#query-syntax)
    -   [Example](#example)
-   [License](#license)

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

This section describes the string query format that can be parsed by the library. The string queries are composed of a series of expressions that represent operations, values, and logical connectors. Expressions can be combined using parentheses to create more complex queries.

#### Operations

*   `==`: Equals
*   `!=`: Not equals
*   `>`: Greater than
*   `>=`: Greater than or equal to
*   `<`: Less than
*   `<=`: Less than or equal to
*   `contains`: Contains substring
*   `startswith`: Starts with substring
*   `endswith`: Ends with substring
*   `regexmatch`: Matches regex pattern
*   `in`: Checks if a value is in a list of values

#### Values

Values can be of the following types:

*   String: Enclosed in double quotes, e.g. `"hello"`
*   Integer: A sequence of digits, e.g. `42`
*   List of strings: Enclosed in square brackets, separated by commas, e.g. `["apple", "banana", "cherry"]`
*   List of integers: Enclosed in square brackets, separated by commas, e.g. `[1, 2, 3]`

#### Logical Connectors

*   `&&`: Logical AND
*   `||`: Logical OR
*   `!`: Logical NOT (used with parentheses)

#### Examples

*   `field1 == "value1"`: field1 equals "value1"
*   `(field1 != "value1") && (field2 > 42)`: field1 is not equal to "value1" and field2 is greater than 42
*   `(field1 contains "substr") || (field2 < 10)`: field1 contains the substring "substr" or field2 is less than 10
*   `!(field1 >= 100)`: field1 is not greater than or equal to 100
*   `field1 in ["apple", "banana"]`: field1 is either "apple" or "banana"
*   `((field1 == "value1") || (field1 == "value2")) && (field2 <= 20)`: field1 is either "value1" or "value2", and field2 is less than or equal to 20

#### Parentheses
To set the precedence of the operations, use parentheses. Expressions enclosed in parentheses will be evaluated first. If you want to group conditions, you can use parentheses to create more complex queries.

For example:

* `((age >= 25) && (interests in ["hiking"])) || (name == "Alice")`: This query will match items where the age is greater than or equal to 25 and the interests contain "hiking", or the name is equal to "Alice".
* `(name == "Alice") || ((age < 30) && (interests in ["cooking"]))`: This query will match items where the name is equal to "Alice", or the age is less than 30 and the interests contain "cooking".

By using parentheses, you can build complex queries that combine multiple conditions with different levels of precedence to achieve precise filtering.

#### Notes

*   Spaces are allowed between elements but are not required.
*   The field names must be valid Rust identifiers.
*   The parser is case-sensitive; field names and string values must match the case exactly.

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
