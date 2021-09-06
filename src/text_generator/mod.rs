use crate::MarkovChainRule;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;

#[cfg(test)]
mod test;

pub fn generate_text(rule: &HashMap<Vec<&str>, Vec<&str>>, length: usize) -> String {
    let mut rng = thread_rng();

    let start = rule.keys().nth(rng.gen_range(0..rule.len())).unwrap();

    let mut chain = start.clone();
    let key_size = chain.len();

    for _ in 0..length {
        let key = &chain[chain.len() - key_size..];
        let nexts = match rule.get(key) {
            None => break,
            Some(e) => e,
        };
        let next = nexts.choose(&mut rng).unwrap();
        chain.push(next);
    }

    chain.join(" ")
}

// Known buggy.
pub fn generate_text_slice(rule: &MarkovChainRule, length: usize) -> String {
    let mut rng = thread_rng();

    let start = rule.keys().nth(rng.gen_range(0..rule.len())).unwrap();

    let mut chain = vec![*start];

    while chain.len() < length {
        let key = &chain[chain.len() - 1..];
        let nexts = match rule.get(key.join("").as_str()) {
            None => break,
            Some(e) => e,
        };
        let next = nexts.choose(&mut rng).unwrap();
        chain.push(next);
    }

    chain.join(" ")
}
