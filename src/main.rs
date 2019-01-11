use std::{collections::HashMap, env, fs};

use rand::{seq::SliceRandom, thread_rng, Rng};

fn read_data(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

type MarkovChainRule<'a> = HashMap<Vec<&'a str>, Vec<&'a str>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let key_size: usize = args[2].parse().expect("Invalid key_size!");
    let output_size: usize = args[3].parse().expect("Invalid output_size!");

    let data = read_data(filename);
    let rule = make_rule(&data, key_size);
    let result = make_string(&rule.unwrap(), output_size);
    println!("{}", result);
}

fn make_rule(content: &str, key_size: usize) -> Result<MarkovChainRule, &'static str> {
    if key_size < 1 {
        return Err("key_size may not be less than 1!");
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

fn make_string(rule: &HashMap<Vec<&str>, Vec<&str>>, length: usize) -> String {
    let mut rng = thread_rng();
    let start = rule.keys().nth(rng.gen_range(0, rule.len())).unwrap();

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
