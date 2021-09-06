use std::collections::HashMap;

use crate::rule_trainer::{
    generate_rule_from_data, generate_rule_from_data_unsafe, generate_rule_from_data_vec,
};

#[test]
fn generate_rule() {
    let content = "Lorem ipsum dolor et ami coloram";
    let rule = generate_rule_from_data(content, 2).unwrap();
    let mut expected = HashMap::new();
    expected.insert("Lorem ipsum", vec!["dolor"]);
    expected.insert("ipsum dolor", vec!["et"]);
    expected.insert("dolor et", vec!["ami"]);
    expected.insert("et ami", vec!["coloram"]);

    assert_eq!(rule, expected);
}

#[test]
fn generate_rule_in_place() {
    let content = "Lorem ipsum   dolor  et ami coloram";
    let rule = generate_rule_from_data(content, 2).unwrap();

    let mut expected = HashMap::new();
    expected.insert("Lorem ipsum", vec!["dolor"]);
    expected.insert("ipsum   dolor", vec!["et"]);
    expected.insert("dolor  et", vec!["ami"]);
    expected.insert("et ami", vec!["coloram"]);

    assert_eq!(rule, expected);
}

#[test]
fn generate_rule_in_place_safe() {
    let content = "Lorem ipsum   dolor  et ami coloram";
    let rule = generate_rule_from_data_unsafe(content, 2).unwrap();

    let mut expected = HashMap::new();
    expected.insert("Lorem ipsum", vec!["dolor"]);
    expected.insert("ipsum   dolor", vec!["et"]);
    expected.insert("dolor  et", vec!["ami"]);
    expected.insert("et ami", vec!["coloram"]);

    assert_eq!(rule, expected);
}

#[test]
fn generate_rule_in_place_with_spaces() {
    let content = "Lorem ipsum   dolor  et ami coloram";
    let rule = generate_rule_from_data(content, 2).unwrap();
    let rule_unsafe = generate_rule_from_data_unsafe(content, 2).unwrap();

    assert_eq!(rule, rule_unsafe);

    let rule_vec = generate_rule_from_data_vec(content, 2).unwrap();
    let rule: HashMap<Vec<&str>, Vec<&str>> = rule
        .iter()
        .map(|(k, v)| (k.split_whitespace().collect(), v.clone()))
        .collect();

    assert_eq!(rule, rule_vec);
}
