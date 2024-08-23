use crate::template::TemplateInfo;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Card<'a> {
    Card(CardInfo<'a>),
    Template(TemplateInfo<'a>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Hero,
    Faction,
    Artifact,
    Wonder,
    Chaos,
    Event,
}

// Remember to use the command pattern for additional effects
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CardInfo<'a> {
    pub card_type: CardType,
    pub year: u32,
    pub name: &'a str,
    pub desc: &'a str,
}

