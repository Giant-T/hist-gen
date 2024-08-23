use rand::{seq::IteratorRandom, Rng};

#[derive(Debug)]
struct MarkovLink {
    pub state: char,
    pub p: u8,
}

#[derive(Debug)]
pub struct MarkovChain {
    nodes: [Vec<MarkovLink>; 26],
}

impl MarkovChain {
    pub fn new() -> Self {
        const ARRAY_REPEAT_VALUE: Vec<MarkovLink> = Vec::new();
        let nodes = [ARRAY_REPEAT_VALUE; 26];
        return MarkovChain { nodes };
    }

    pub fn push_link(&mut self, start_state: char, next_state: char, p: u8) {
        let link = MarkovLink {
            state: next_state,
            p,
        };
        self.nodes[(start_state as u8 - b'a') as usize].push(link);
    }

    pub fn next_state(&self, state: char, x: u8) -> char {
        let links = &self.nodes[(state as u8 - b'a') as usize];

        let mut p = 0;
        for link in links {
            p += link.p;
            if x <= p {
                return link.state;
            }
        }

        return '_';
    }
}

/// Generates a name using a Markov Chain
pub fn generate_character_name(chain: &MarkovChain) -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::new();

    let mut state = ('a'..='z').choose(&mut rng).unwrap();

    name.push(state.to_ascii_uppercase());

    let mut should_end = false;
    while (!should_end || name.len() <= 3) && name.len() <= 12 {
        let s = chain.next_state(state, rng.gen());
        should_end = s == '_';

        if !should_end {
            state = s;
            name.push(state);
        }
    }

    return name;
}

pub fn parse_markov_file() -> MarkovChain {
    let file = include_str!("../ressources/from_data.txt");

    let mut chain = MarkovChain::new();

    file.lines().for_each(|line| {
        let mut split = line.split(' ');
        let start_state = split.next().unwrap().chars().next().unwrap();
        let next_state = split.next().unwrap().chars().next().unwrap();
        let p = u8::from_str_radix(split.next().unwrap(), 10).unwrap();

        chain.push_link(start_state, next_state, p);
    });

    return chain;
}

#[test]
/// Verifies that all probabilities add up to 1
fn markov_chain_verification() {
    let chain = parse_markov_file();

    let p: std::collections::HashMap<char, u8> = chain
        .nodes
        .iter()
        .enumerate()
        .map(|(i, links)| {
            (
                (b'a' + i as u8) as char,
                links.iter().map(|link| link.p).sum(),
            )
        })
        .filter(|x| x.1 != 255)
        .collect();

    println!("{p:?}");

    let all_correct = chain
        .nodes
        .iter()
        .map(|links| links.iter().map(|link| link.p).sum())
        .all(|p: u8| p == 255);

    assert!(all_correct);
}
