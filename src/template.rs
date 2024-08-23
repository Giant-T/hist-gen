use core::str;
use std::{
    fs::File,
    io::{self, Read},
};

use crate::{
    card::{Card, CardType},
    toml::parse_toml,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TemplateInfo<'a> {
    pub card_type: CardType,
    pub title: &'a str,
    pub desc: &'a str,
}

pub fn parse_template(path: &str) -> Result<Card, io::Error> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut template_info = TemplateInfo {
        card_type: CardType::Event,
        title: "",
        desc: "",
    };

    let parsed = parse_toml(&content);
    println!("{parsed:?}");

    return Ok(Card::Template(template_info));
}
