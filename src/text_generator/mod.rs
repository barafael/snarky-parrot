use crate::MarkovChainRule;
use rand::{prelude::IteratorRandom, seq::SliceRandom, thread_rng};
use std::collections::HashMap;

#[cfg(test)]
mod test;

pub fn generate_text(rule: &HashMap<Vec<&str>, Vec<&str>>, length: usize) -> String {
    if rule.is_empty() {
        return String::new();
    }

    let mut rng = thread_rng();

    // unwrap: rule is checked for emptiness above.
    let start = rule.keys().choose(&mut rng).unwrap();

    let mut chain = start.clone();
    let key_size = chain.len();

    for _ in 0..length {
        // get `key_size` last elements of `chain`.
        let key = &chain[chain.len() - key_size..];
        let candidates = match rule.get(key) {
            None => break,
            Some(e) => e,
        };
        let next = candidates.choose(&mut rng).unwrap();
        chain.push(next);
    }

    chain.join(" ")
}

// Known buggy.
pub fn generate_text_slice(rule: &MarkovChainRule, length: usize) -> String {
    if rule.is_empty() {
        return String::new();
    }

    let mut rng = thread_rng();

    // unwrap: rule is checked for emptiness above.
    let start = rule.keys().choose(&mut rng).unwrap();

    let mut chain = vec![*start];

    while chain.len() < length {
        let key = *chain.last().unwrap();
        let candidates = match rule.get(key) {
            None => break,
            Some(e) => e,
        };
        let next = candidates.choose(&mut rng).unwrap();
        chain.push(next);
    }

    chain.join(" ")
}
