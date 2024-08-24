use std::str::FromStr;

pub mod template;

use template::TemplateInfo;

#[derive(Debug, PartialEq)]
pub enum Card {
    Card(CardInfo),
    Template(TemplateInfo),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Artifact,
    Chaos,
    Event,
    Faction,
    Hero,
    Wonder,
}

#[derive(Debug)]
pub struct CardTypeError;

impl FromStr for CardType {
    type Err = CardTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Artifact" => Ok(Self::Artifact),
            "Chaos" => Ok(Self::Chaos),
            "Event" => Ok(Self::Event),
            "Faction" => Ok(Self::Faction),
            "Hero" => Ok(Self::Hero),
            "Wonder" => Ok(Self::Wonder),
            _ => Err(CardTypeError),
        }
    }
}

// Remember to use the command pattern for additional effects
#[derive(Debug, PartialEq)]
pub struct CardInfo {
    pub card_type: CardType,
    pub year: u32,
    pub name: String,
    pub desc: String,
}
