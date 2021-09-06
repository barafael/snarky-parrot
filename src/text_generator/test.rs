use crate::{
    rule_trainer::{generate_rule_from_data, generate_rule_from_data_vec},
    text_generator::generate_text_slice,
};

use super::generate_text;

#[test]
fn generate_text_trivially() {
    let content = "0 1 2 3 4 5 6 7 8 9";
    let rule = generate_rule_from_data_vec(content, 1).unwrap();
    let text = generate_text(&rule, 100);

    assert!(content.contains(&text));
}

#[test]
fn generate_text_trivially_slice() {
    let content = "0 1 2 3 4 5 6 7 8 9";
    let rule = generate_rule_from_data(content, 1).unwrap();
    let text = generate_text_slice(&rule, 100);

    dbg!(&text);
    assert!(content.contains(&text));
}
