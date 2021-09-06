use criterion::{black_box, criterion_group, criterion_main, Criterion};
use snarky_parrot::{
    rule_trainer::{
        generate_rule_from_data, generate_rule_from_data_btreemap,
        generate_rule_from_data_hashbrown_safe, generate_rule_from_data_hashbrown_unsafe,
        generate_rule_from_data_unsafe, generate_rule_from_data_vec,
        generate_rule_from_data_vec_hashbrown,
    },
    text_generator::{generate_text, generate_text_slice},
};

const BIBLE: &str = include_str!("bible.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let len = 10000;
    let content =
        std::str::from_utf8(unsafe { std::slice::from_raw_parts(BIBLE.as_ptr(), len) }).unwrap();

    c.bench_function("rule generation vec", |b| {
        b.iter(|| generate_rule_from_data_vec(black_box(content), 4))
    });

    c.bench_function("rule generation safe", |b| {
        b.iter(|| generate_rule_from_data(black_box(content), 4))
    });

    c.bench_function("rule generation unsafe", |b| {
        b.iter(|| generate_rule_from_data_unsafe(black_box(content), 4))
    });

    c.bench_function("rule generation safe btreemap", |b| {
        b.iter(|| generate_rule_from_data_btreemap(black_box(content), 4))
    });

    c.bench_function("rule generation safe hashbrown", |b| {
        b.iter(|| generate_rule_from_data_hashbrown_safe(black_box(content), 4))
    });

    c.bench_function("rule generation unsafe hashbrown", |b| {
        b.iter(|| generate_rule_from_data_hashbrown_unsafe(black_box(content), 4))
    });

    c.bench_function(
        "rule generation with hashbrown and original function",
        |b| b.iter(|| generate_rule_from_data_vec_hashbrown(black_box(content), 4)),
    );

    let rule = generate_rule_from_data_vec(content, 4).unwrap();
    c.bench_function("text generation vec", |b| {
        b.iter(|| generate_text(&rule, 1000))
    });

    let rule = generate_rule_from_data(content, 4).unwrap();
    c.bench_function("text generation slice", |b| {
        b.iter(|| generate_text_slice(&rule, 1000))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
