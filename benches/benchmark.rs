use criterion::{black_box, criterion_group, criterion_main, Criterion};
use snarky_parrot::rule_trainer::{
    generate_rule_from_data, generate_rule_from_data_in_place,
    generate_rule_from_data_in_place_safe,
};

const BIBLE: &str = include_str!("bible.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let len = 10000;
    let content =
        std::str::from_utf8(unsafe { std::slice::from_raw_parts(BIBLE.as_ptr(), len) }).unwrap();

    c.bench_function("rule generation 1", |b| {
        b.iter(|| generate_rule_from_data(black_box(content), 4))
    });

    c.bench_function("rule generation 2", |b| {
        b.iter(|| generate_rule_from_data_in_place(black_box(content), 4))
    });

    c.bench_function("rule generation 3", |b| {
        b.iter(|| generate_rule_from_data_in_place_safe(black_box(content), 4))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
