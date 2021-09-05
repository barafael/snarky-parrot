use std::collections::HashMap;

use crate::generate_rule_from_data;

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
