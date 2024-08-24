use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug)]
pub enum TomlType {
    String(String),
    Boolean(bool),
    Array(Vec<TomlType>),
    Table(HashMap<String, TomlType>),
    Integer(i32),
    Float(f64),
}

#[derive(Debug)]
pub enum TomlError {
    InvalidToml,
    ParsingError,
}

type TomlResult<T> = Result<T, TomlError>;

impl Error for TomlError {}

impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in TOML: wrong syntax or unimplemented features.")
    }
}

/// Parses toml content (alternative number formats, dates and string escape sequences are not supported (._.) )
pub fn parse_toml(content: &str) -> TomlResult<HashMap<String, TomlType>> {
    let mut values = HashMap::new();

    // TODO: implement sections
    let mut start: usize = 0;
    let mut end: usize = 0;

    while end < content.len() {
        let mut looking_for = "\n";
        while end < content.len() && !content[..=end].ends_with(looking_for) {
            if looking_for != "\"\"\"\n" && content[..=end].ends_with("\"\"\"") {
                looking_for = "\"\"\"\n";
                end += 1;
            } else if content[..=end].ends_with('{') {
                looking_for = "}\n";
            } else if content[..=end].ends_with('[') {
                looking_for = "]\n";
            }
            end += 1;
        }
        let line = &content[start..=end];

        end += 1;
        start = end;

        let should_skip = line.starts_with('#') || line.trim().is_empty();
        if should_skip {
            continue;
        }

        let entry = parse_toml_table_entry(line)?;

        if let Some(_) = values.insert(entry.0, entry.1) {
            return Err(TomlError::InvalidToml);
        }
    }

    return Ok(values);
}

/// Parses a toml entry
/// ex:
/// &nbsp; ident = value
fn parse_toml_table_entry(str: &str) -> TomlResult<(String, TomlType)> {
    if !str.contains('=') {
        return Err(TomlError::InvalidToml);
    }

    let split = str.split_once('=').unwrap();
    let ident = split.0.trim();
    let value = parse_toml_value(split.1.trim())?;

    return Ok((ident.into(), value));
}

/// Parses a toml value depending on its type
fn parse_toml_value(str: &str) -> TomlResult<TomlType> {
    if str.starts_with("\"\"\"") {
        if !str.ends_with("\"\"\"") {
            return Err(TomlError::InvalidToml);
        }
        return Ok(TomlType::String(parse_multiline_str(str)));
    } else if str.starts_with('"') {
        if !str.ends_with('"') {
            return Err(TomlError::InvalidToml);
        }
        let end = str.len() - 1;
        return Ok(TomlType::String(str[1..end].into()));
    } else if str.starts_with('[') {
        if !str.ends_with(']') {
            return Err(TomlError::InvalidToml);
        }

        // Parse each element of the array
        let end = str.len() - 1;
        let arr = str[1..end]
            .split(',')
            .map(|x| parse_toml_value(x.trim()))
            .collect::<TomlResult<Vec<_>>>()?;

        return Ok(TomlType::Array(arr));
    } else if str.starts_with('{') {
        if !str.ends_with('}') {
            return Err(TomlError::InvalidToml);
        }

        // Parse each element of the table
        let end = str.len() - 1;
        let table = str[1..end]
            .split(',')
            .map(|x| parse_toml_table_entry(x.trim()))
            .collect::<TomlResult<HashMap<_, _>>>()?;

        return Ok(TomlType::Table(table));
    } else if let Ok(int) = str.parse() {
        return Ok(TomlType::Integer(int));
    } else if let Ok(float) = str.parse() {
        return Ok(TomlType::Float(float));
    }

    return Err(TomlError::ParsingError);
}

/// Parses a multiline string
fn parse_multiline_str(str: &str) -> String {
    let end = str.len() - 3;

    str[3..end]
        .lines()
        .map(|line| {
            let line = line.trim_start();
            if line.ends_with('\\') {
                let end = line.len() - 1;
                return line[..end].into();
            }
            let mut l = line.to_string();
            l.push('\n');
            return l;
        })
        .collect()
}

// TODO: make up some parser tests
