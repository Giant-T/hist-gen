#![allow(dead_code)]

use name_generator::{generate_character_name, parse_markov_file};

mod card;
mod name_generator;
mod narrator;

fn main() {
    let chain = parse_markov_file();
    let name = generate_character_name(&chain);
    println!("{}", name);
}
