use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use vec_filter::{parse_query, Filterable};

#[derive(Debug, Clone, PartialEq, Filterable)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub interests: Vec<String>,
}

fn bench_parse_query(c: &mut Criterion) {
    let query = "((name == \"Alice\") && (interests in [\"hiking\"])) || (age == 20)";
    let mut group = c.benchmark_group("parse_query");
    group.throughput(Throughput::Elements(1));
    group.bench_function("ips", |b| {
        b.iter(|| parse_query::<PersonProperties>(black_box(query)))
    });
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

    let people = vec![alice.clone(), bob.clone(), carol.clone()];
    let query = "((name == \"Alice\") && (interests in [\"hiking\"])) || (age == 20)";
    let ast = parse_query(query).unwrap();

    let mut group = c.benchmark_group("apply");
    group.throughput(Throughput::Elements(3));
    group.bench_function("ips", |b| {
        b.iter(|| ast.1.apply(black_box(&people)))
    });
    group.finish();
}

criterion_group!(benches, bench_parse_query, bench_apply);
criterion_main!(benches);