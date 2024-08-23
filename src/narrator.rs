use crate::{
    card::Card,
    name_generator::{parse_markov_file, MarkovChain},
};

#[derive(Debug)]
pub struct Narrator<'a> {
    year: u32,
    history: String,
    hand: [Option<Box<Card<'a>>>; 10],
    discarded: Vec<Box<Card<'a>>>,
    deck: Vec<Box<Card<'a>>>,
    pub chain: MarkovChain,
}

impl<'a> Narrator<'a> {
    pub fn new() -> Self {
        Narrator {
            year: 0,
            history: String::new(),
            hand: [None, None, None, None, None, None, None, None, None, None],
            deck: Vec::new(),
            discarded: Vec::new(),
            chain: parse_markov_file(),
        }
    }

    pub fn play_turn(self: &mut Self) {
        // picks a card from the deck if hand is not full
        if self.hand.contains(&None) {
            self.pick_cards(1);
        }
        // plays a card from his hand and discards it
        // writes history
        // advance the time accordingly
    }

    fn pick_cards(self: &mut Self, _n: u8) {
        // picks n cards from the top of the deck
    }

    fn search_deck(self: &mut Self, _card_name: &str) -> Option<Card> {
        // search for a card in the deck and returns it
        // if it can be found and removes it from the deck
        return None;
    }

    fn play_cards(self: &mut Self) {
        // pick card randomly from hand play its effects
        // then discard it
    }

    fn discard_cards(self: &mut Self, _n: u8) {
        // discards n cards randomly
    }
}
