use std::collections::HashMap;

#[derive(Debug)]
pub enum TomlType {
    String(String),
    Boolean(bool),
    Array(Vec<TomlType>),
    Table(HashMap<String, TomlType>),
    Integer(u32),
    Float(f64),
}

pub fn parse_toml(content: &str) -> HashMap<String, TomlType> {
    let mut values = HashMap::new();

    content.lines().for_each(|line| {
        if line.starts_with('#') {
            return;
        }

        let entry = parse_toml_table_entry(line).unwrap();

        values.insert(entry.0, entry.1);
    });
    return values;
}

fn parse_toml_table_entry(str: &str) -> Result<(String, TomlType), ()> {
    if !str.contains('=') {
        return Err(());
    }

    let split = str.split_once('=').unwrap();
    let ident = split.0.trim();
    let value = parse_toml_value(split.1.trim()).unwrap();

    return Ok((ident.into(), value));
}

fn parse_toml_value(str: &str) -> Result<TomlType, ()> {
    if str.starts_with('"') && str.ends_with('"') {
        let end = str.len() - 1;
        return Ok(TomlType::String(str[1..end].into()));
    } else if str.starts_with('[') && str.ends_with(']') {
        // Parse each element of the array
        let end = str.len() - 1;
        let arr = str[1..end]
            .split(',')
            .map(|x| parse_toml_value(x.trim()).unwrap())
            .collect();

        return Ok(TomlType::Array(arr));
    } else if str.starts_with('{') && str.ends_with('}') {
        let end = str.len() - 1;
        let table = str[1..end]
            .split(',')
            .map(|x| parse_toml_table_entry(x.trim()).unwrap())
            .collect();

        return Ok(TomlType::Table(table));
    } else if let Ok(int) = str.parse::<u32>() {
        return Ok(TomlType::Integer(int));
    } else if let Ok(float) = str.parse::<f64>() {
        return Ok(TomlType::Float(float));
    }

    return Err(());
}
