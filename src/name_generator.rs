use rand::{seq::IteratorRandom, Rng};

struct MarkovLink {
    pub state: char,
    pub p: u8,
}

struct MarkovChain {
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
pub fn generate_character_name() -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::new();

    let mut state = ('a'..'z').choose(&mut rng).unwrap();

    let chain = parse_markov_file();

    name.push(state.to_ascii_uppercase());

    while state != '_' {
        state = chain.next_state(state, rng.gen());
        name.push(state);
    }

    return name;
}

fn parse_markov_file() -> MarkovChain {
    let file = include_str!("../markov_nodes.txt");

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
