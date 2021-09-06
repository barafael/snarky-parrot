use std::collections::HashMap;

mod error;

pub mod rule_trainer;
pub mod text_generator;

pub type MarkovChainRule<'a> = HashMap<&'a str, Vec<&'a str>>;
