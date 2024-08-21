use name_generator::generate_character_name;

mod narrator;
mod card;
mod name_generator;

fn main() {
    let name = generate_character_name();
    println!("{}", name);
}
