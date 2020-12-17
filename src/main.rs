use std::env;
use std::fs;

use markov_chain_text_generator::make_rule;
use markov_chain_text_generator::make_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let key_size: usize = args[2].parse().expect("Invalid key_size!");
    let output_size: usize = args[3].parse().expect("Invalid output_size!");

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let rule = make_rule(&data, key_size);
    let result = make_string(&rule.unwrap(), output_size);
    println!("{}", result);
}
