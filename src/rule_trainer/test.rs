use std::collections::HashMap;

use crate::rule_trainer::{
    generate_rule_from_data, generate_rule_from_data_in_place,
    generate_rule_from_data_in_place_safe,
};

#[test]
fn generate_rule() {
    let content = "Lorem ipsum dolor et ami coloram";
    let rule = generate_rule_from_data(content, 2).unwrap();
    let mut expected = HashMap::new();
    expected.insert(vec!["Lorem", "ipsum"], vec!["dolor"]);
    expected.insert(vec!["ipsum", "dolor"], vec!["et"]);
    expected.insert(vec!["dolor", "et"], vec!["ami"]);
    expected.insert(vec!["et", "ami"], vec!["coloram"]);

    assert_eq!(rule, expected);
}

#[test]
fn generate_rule_in_place() {
    let content = "Lorem ipsum   dolor  et ami coloram";
    let rule = generate_rule_from_data_in_place(content, 2).unwrap();

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
    let rule = generate_rule_from_data_in_place_safe(content, 2).unwrap();

    let mut expected = HashMap::new();
    expected.insert("Lorem ipsum", vec!["dolor"]);
    expected.insert("ipsum   dolor", vec!["et"]);
    expected.insert("dolor  et", vec!["ami"]);
    expected.insert("et ami", vec!["coloram"]);

    assert_eq!(rule, expected);
}
