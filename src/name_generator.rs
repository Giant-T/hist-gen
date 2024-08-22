use std::collections::HashMap;

use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

/// Generates a name using Markov Chains
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

    return name;
}

/// Randomly picks a syllable from a list
fn generate_name_syllables(rng: &mut ThreadRng) -> &str {
    let syllables = include_str!("../name_syllables.txt");
    return syllables
        .lines()
        .choose(rng)
        .expect("Could not choose syllable randomly.");
}

#[test]
/// Verifies that all probabilities add up to 1
fn markov_chain_verification() {
    let file = include_str!("../markov_nodes.txt");
    let mut nodes: [u8; 26] = [0; 26];

    file.lines().for_each(|line| {
        let mut split = line.split(' ');
        let letter = split.next().unwrap().as_bytes()[0];
        let p: u8 = u8::from_str_radix(split.next_back().unwrap(), 10).unwrap();

        nodes[(letter - b'a') as usize] += p;
    });

    let all_correct = nodes.iter().all(|p| *p == 255);

    assert!(all_correct);
}
