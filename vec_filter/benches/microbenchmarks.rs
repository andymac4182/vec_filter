use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use vec_filter::{parse_query, Filterable};

#[derive(Debug, Clone, PartialEq, Filterable)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub interests: Vec<String>,
}

fn bench_parse_query(c: &mut Criterion) {
    let input_strings = vec![
        "name == \"Alice\"",
        "name != \"Alice\"",
        "age == 30",
        "age != 30",
        "interests in [\"reading\"]",
        "interests in [\"cooking\"]",
        "interests in [\"hiking\"]",
        "(name == \"Alice\") && (age == 30)",
        "(name == \"Alice\") || (name == \"Bob\")",
        "name in [\"Alice\",\"Bob\"]",
        "interests in \"hiking\"",
        "(name == \"Alice\") || (name == \"Bob\") || (name == \"Eve\")",
        "(interests in [\"hiking\"]) && (age == 25)",
        "((name == \"Alice\") && (interests in [\"hiking\"])) || (age == 20)",
        "(interests in [\"hiking\"]) && ((age == 20) || (age == 25))",
        "age > 25",
        "age < 25",
        "age >= 25",
        "age <= 25",
    ];
    let mut group = c.benchmark_group("parse_query");
    for query in input_strings.iter() {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::from_parameter(query), query, |b, &query| {
            b.iter(|| parse_query::<PersonProperties>(black_box(query)))
        });
    }
    group.finish();
}

fn bench_apply(c: &mut Criterion) {
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
    let query = "((name == \"Alice\") && (interests in [\"hiking\"])) || (age == 20)";
    let ast = parse_query(query).unwrap();

    let mut group = c.benchmark_group("apply");
    group.throughput(Throughput::Elements(3));
    group.bench_function("ips", |b| b.iter(|| ast.apply(black_box(&people))));
    group.finish();
}

criterion_group!(benches, bench_parse_query, bench_apply);
criterion_main!(benches);
