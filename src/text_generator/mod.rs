use crate::MarkovChainRule;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn generate_text(rule: &MarkovChainRule, length: usize) -> String {
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
