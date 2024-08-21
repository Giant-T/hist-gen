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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CardInfo<'a> {
    pub card_type: CardType,
    pub year: u32,
    pub name: &'a str,
    pub desc: &'a str,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TemplateInfo<'a> {
    pub card_type: CardType,
    pub desc: &'a str,
}

pub fn parse_template(file_content: &str) -> Card {
    return Card::Template(
        TemplateInfo { card_type: CardType::Event, desc: "" }
    );
}

