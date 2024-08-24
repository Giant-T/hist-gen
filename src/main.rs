#![allow(dead_code)]

use std::{error::Error, time::SystemTime};

use name_generator::generate_character_name;
use narrator::Narrator;
use card::template::parse_template;

mod card;
mod name_generator;
mod narrator;
mod toml;

fn main() -> Result<(), Box<dyn Error>> {
    let narrator = Narrator::new();
    let name = generate_character_name(&narrator.chain);
    println!("{}", name);

    let time = SystemTime::now();
    let template = parse_template("ressources/templates/t1.toml")?;

    println!("{:?} : {template:?}", time.elapsed()?);

    Ok(())
}
