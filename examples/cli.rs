use std::env;
use std::fs;

use markov_chain_text_generator::generate_rule_from_data;
use markov_chain_text_generator::generate_text;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please provide input file name");
    let key_size: usize = args
        .get(2)
        .expect("Please provide key size (up to a couple dozen or so, then it gets slow)")
        .parse()
        .expect("Invalid key_size!");
    let output_size: usize = args
        .get(3)
        .expect("Please provide lenght of output text")
        .parse()
        .expect("Invalid output_size!");

    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let rule = generate_rule_from_data(&data, key_size).expect("Failed to generate rules");
    let result = generate_text(&rule, output_size);
    println!("{}", result);
}
