use std::{collections::HashMap, error::Error, fmt, fs::File, io::Read, path::Path, str::FromStr};

use crate::{
    card::{Card, CardType},
    toml::{parse_toml, TomlType},
};

#[derive(Debug, PartialEq)]
pub struct TemplateInfo {
    pub card_type: CardType,
    pub title: String,
    pub desc: String,
}

#[derive(Debug)]
pub struct TemplateError;

impl Error for TemplateError {}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in template: Missing field(s) or wrong type(s).")
    }
}

impl TryFrom<HashMap<String, TomlType>> for TemplateInfo {
    type Error = TemplateError;

    fn try_from(value: HashMap<String, TomlType>) -> Result<Self, Self::Error> {
        let TomlType::String(title) = value.get("title").ok_or(TemplateError)? else {
            return Err(TemplateError);
        };
        let TomlType::String(desc) = value.get("description").ok_or(TemplateError)? else {
            return Err(TemplateError);
        };
        let TomlType::String(card_type) = value.get("type").ok_or(TemplateError)? else {
            return Err(TemplateError);
        };

        let card_type = CardType::from_str(card_type.as_str()).or(Err(TemplateError))?;

        Ok(TemplateInfo {
            title: title.to_string(),
            desc: desc.to_string(),
            card_type,
        })
    }
}

pub fn parse_template<P>(path: P) -> Result<Card, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let template_info = parse_toml(&content)?.try_into()?;

    return Ok(Card::Template(template_info));
}

// TODO: Make up template parsing tests
