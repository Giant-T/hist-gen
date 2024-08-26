use std::{collections::HashMap, error::Error, fmt};

use iterator::TomlIterator;

pub mod iterator;

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

/// Parses toml content (alternative number formats, dates and string escape sequences are not supported (._.))
/// Also, comments can only be found at the start of a line
pub fn parse_toml(content: &str) -> TomlResult<HashMap<String, TomlType>> {
    let mut iter = TomlIterator::new(content);

    let mut values: HashMap<String, TomlType> = HashMap::new();

    iter.try_for_each(|x| {
        let (ident, value) = x?;

        let entry = parse_toml_table_entry(ident, value)?;

        if values.contains_key(&entry.0) {
            return Err(TomlError::InvalidToml);
        }

        values.insert(entry.0, entry.1);

        Ok(())
    })?;

    return Ok(values);
}

/// Parses a toml entry
fn parse_toml_table_entry(ident: &str, value: &str) -> TomlResult<(String, TomlType)> {
    if ident.starts_with('[') && ident.ends_with(']') {
        let ident = &ident[1..(ident.len() - 1)];
        let mut iter = TomlIterator::new(value);
        let mut values: HashMap<String, TomlType> = HashMap::new();
        iter.try_for_each(|x| {
            let (ident, value) = x?;

            if values.contains_key(ident) {
                return Err(TomlError::InvalidToml);
            }

            values.insert(ident.into(), parse_toml_value(value)?);

            Ok(())
        })?;
        return Ok((ident.into(), TomlType::Table(values)));
    }

    let value = parse_toml_value(value)?;

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
        if str.len() == 1 || !str.ends_with('"') || str.contains('\n') {
            return Err(TomlError::InvalidToml);
        }
        let end = str.len() - 1;
        return Ok(TomlType::String(str[1..end].into()));
    } else if str.starts_with('[') {
        if !str.ends_with(']') {
            return Err(TomlError::InvalidToml);
        }

        // TODO: use a stack of closing and opening commas to split entries of
        //      tables and arrays

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
            .filter(|x| !x.trim().is_empty())
            .map(|x| {
                let Some(split) = x.split_once('=') else {
                    return Err(TomlError::InvalidToml);
                };

                let val = parse_toml_value(split.1.trim())?;
                return Ok((split.0.trim().into(), val));
            })
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
