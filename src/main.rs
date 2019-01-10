use std::env;
use std::fs;
 
use std::collections::HashMap;
 
extern crate rand;
use rand::{Rng, thread_rng};
 
fn read_data(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
 
type MarkovChainRule = HashMap<Vec<String>, Vec<String>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let key_size = args[2].parse::<usize>().expect("Invalid key_size!");
    let output_size = args[3].parse::<usize>().expect("Invalid output_size!");
 
    let data = read_data(filename);
    let rule = make_rule(&data, key_size);
    let result = make_string(&rule.unwrap(), output_size);
    println!("{}", result);
}
 
fn make_rule(content: &str, key_size: usize) -> Result<MarkovChainRule, &'static str> {
    if key_size < 1 {
        eprintln!();
        return Err("key_size may not be less than 1!");
    }
    let words: Vec<&str> = content.split(' ').collect();
 
    let mut dict: MarkovChainRule = HashMap::new();
 
    for i in 0..words.len() - key_size {
        let mut key = vec![words[i].to_string()];
        for word in words.iter().take(i + key_size).skip(i + 1) {
            key.push(word.to_string());
        }
        let value = words[i + key_size];
        match dict.get_mut(&key) {
            Some(e) => {
                e.push(value.to_string());
            }
            None => {
                dict.insert(key, vec![value.to_string()]);
            }
        }
    }
    Ok(dict)
}
 
fn make_string(rule: &MarkovChainRule, length: usize) -> String {
    let keys: Vec<&Vec<String>> = rule.keys().collect();
    let mut words = get_random_element(&keys).to_vec();
    let mut buffer = words.clone().join(" ");
    buffer.push_str(" ");
    for _i in 0..length {
        let entry = match rule.get(&words[..]) {
            None => continue,
            Some(e) => e,
        };
        let new = get_random_element(entry);
        buffer.push_str(new);
        buffer.push_str(" ");
        let len = words.len();
        for j in 0..len - 1 {
            words[j] = words[j + 1].clone();
        }
        words[len - 1] = new.to_string();
    }
    buffer
}
 
fn get_random_element<T>(slice: &[T]) -> &T {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, slice.len());
    &slice[index]
}

