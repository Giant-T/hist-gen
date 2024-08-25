use std::{cmp::min, collections::HashMap, error::Error, fmt};

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
/// , also comments can only be found at the start of a line
pub fn parse_toml(content: &str) -> TomlResult<HashMap<String, TomlType>> {
    let mut values = HashMap::new();

    // TODO: implement sections
    let mut start: usize = 0;
    let mut end: usize = 0;

    let mut section: Option<&str> = None;

    while end < content.len() {
        if content[start..=end].starts_with('[') {
            // Parse section header
            while end < content.len() && !content[..=end].ends_with("]\n") {
                end += 1;
            }

            section = Some(&content[(start + 1)..(end - 1)]);
            if let Some(_) = values.insert(
                content[(start + 1)..(end - 1)].into(),
                TomlType::Table(HashMap::new()),
            ) {
                return Err(TomlError::InvalidToml);
            }
            end = min(content.len() - 1, end + 1);
            start = end;
        }

        let mut looking_for = "\n";
        while end < content.len() - 1 && !content[..=end].ends_with(looking_for) {
            if looking_for != "\"\"\"\n" && content[..=end].ends_with("\"\"\"") {
                looking_for = "\"\"\"\n";
                end = min(content.len() - 1, end + 1);
            } else if content[..=end].ends_with('{') {
                looking_for = "}\n";
            } else if content[..=end].ends_with('[') {
                looking_for = "]\n";
            }
            end = min(content.len() - 1, end + 1);
        }

        let line = &content[start..=end];

        end += 1;
        start = end;

        let should_skip = line.starts_with('#') || line.trim().is_empty();
        if should_skip {
            continue;
        }

        let entry = parse_toml_table_entry(line)?;

        // If in a section insert into this section
        if let Some(s) = section {
            if let TomlType::Table(table) = values.get_mut(s).unwrap() {
                if let Some(_) = table.insert(entry.0, entry.1) {
                    return Err(TomlError::InvalidToml);
                }
                continue;
            }
        }

        if let Some(_) = values.insert(entry.0, entry.1) {
            return Err(TomlError::InvalidToml);
        }
    }

    return Ok(values);
}

/// Parses a toml entry
/// ex:
///     ident = value
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
        if str.len() <= 6 || !str.ends_with("\"\"\"") {
            return Err(TomlError::InvalidToml);
        }
        return Ok(TomlType::String(parse_multiline_str(str)));
    } else if str.starts_with('"') {
        if str.len() == 1 || !str.ends_with('"') {
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
    let mut string = String::new();
    let mut lines = str[3..].lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim_start();
        if line.ends_with('\\') {
            let end = line.len() - 1;
            string.push_str(line[..end].into());
            continue;
        }
        if lines.peek().is_none() {
            string.push_str(&line[..(line.len() - 3)]);
            continue;
        }
        // Add newline if no '\'
        string.push_str(line);
        string.push('\n');
    }

    return string;
}

// TODO: make up some parser tests
#[cfg(test)]
mod test {
    use crate::toml::TomlType;

    use super::parse_toml;
    use std::{
        fs::{read_dir, File},
        io::Read,
    };

    #[test]
    fn test_wrong_templates() {
        let paths = read_dir("ressources/templates/tests/invalid-syntax").unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let mut content = String::new();
            let mut file = File::open(&path).unwrap();
            file.read_to_string(&mut content).unwrap();
            let t = parse_toml(&content);
            println!("{t:?}");
            assert!(t.is_err());
        }
    }

    #[test]
    fn test_string_templates() {
        let mut content = String::new();
        let mut file = File::open("ressources/templates/tests/valid/string.toml").unwrap();
        file.read_to_string(&mut content).unwrap();

        let toml = parse_toml(&content).unwrap();

        let TomlType::String(t) = toml.get("t1").unwrap() else {
            return assert!(false);
        };
        assert_eq!("", t);

        let TomlType::String(t) = toml.get("t2").unwrap() else {
            return assert!(false);
        };
        assert_eq!("1", t);

        let TomlType::String(t) = toml.get("t3").unwrap() else {
            return assert!(false);
        };
        assert_eq!("test", t);

        let TomlType::String(t) = toml.get("t4").unwrap() else {
            return assert!(false);
        };
        assert_eq!("\ntest\n", t);

        let TomlType::String(t) = toml.get("t5").unwrap() else {
            return assert!(false);
        };
        assert_eq!("test", t);
    }
}
