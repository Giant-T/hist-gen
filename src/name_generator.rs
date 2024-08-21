use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

/// Generates a character name with a random amount of syllables
pub fn generate_character_name() -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::new();

    name.push_str(generate_name_syllables(&mut rng));

    let mut c = name.chars();
    let first_char = c.next().unwrap();

    name = first_char.to_uppercase().chain(c).collect();

    for _ in 0..(rng.gen_range(1..5)) {
        name.push_str(generate_name_syllables(&mut rng));
    }

    return name
}

/// Randomly picks a syllable from a list
fn generate_name_syllables(rng: &mut ThreadRng) -> &str {
    let syllables = include_str!("../name_syllables.txt");
    return syllables
        .lines()
        .choose(rng)
        .expect("Could not choose syllable randomly.");
}
