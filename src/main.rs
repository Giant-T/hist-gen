#![allow(dead_code)]

use name_generator::generate_character_name;
use narrator::Narrator;
use template::parse_template;

mod card;
mod name_generator;
mod narrator;
mod template;
mod toml;

fn main() {
    let narrator = Narrator::new();
    let name = generate_character_name(&narrator.chain);
    println!("{}", name);

    let template = parse_template("ressources/templates/t1.toml").unwrap();
    println!("{template:?}");
}
