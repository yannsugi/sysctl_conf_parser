use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Map(HashMap<String, Value>),
}

type Config = HashMap<String, Value>;

fn insert_into_map(map: &mut Config, keys: &[&str], value: String) {
    if keys.is_empty() {
        return;
    }

    let key = keys[0].to_string();
    if keys.len() == 1 {
        map.insert(key, Value::String(value));
    } else {
        let entry = map.entry(key).or_insert_with(|| Value::Map(HashMap::new()));
        if let Value::Map(sub_map) = entry {
            insert_into_map(sub_map, &keys[1..], value);
        }
    }
}

fn parse_sysctl_file(filename: &str) -> io::Result<Config> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut config = Config::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            let keys: Vec<&str> = key.split('.').collect();
            insert_into_map(&mut config, &keys, value.to_string());
        }
    }
    Ok(config)
}

pub fn parse_sysctl_from_path(path: &str) -> io::Result<Config> {
    parse_sysctl_file(path)
}
