use error::Error;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;

mod error;

pub type MarkovChainRule<'a> = HashMap<Vec<&'a str>, Vec<&'a str>>;

pub fn generate_rule_from_data(content: &str, key_size: usize) -> Result<MarkovChainRule, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: MarkovChainRule = HashMap::new();

    for slice in words.windows(key_size + 1) {
        let (key, value) = slice.split_at(key_size);
        let value = value[0];
        match dict.get_mut(key) {
            Some(e) => {
                e.push(value);
            }
            None => {
                dict.insert(key.to_vec(), vec![value]);
            }
        }
    }

    Ok(dict)
}

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
