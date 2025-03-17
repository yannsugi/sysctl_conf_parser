use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Map(HashMap<String, Value>),
}

#[derive(Debug, Default)]
#[allow(dead_code)] // self.0にアクセスしないため、警告を抑制
pub struct SysctlConf(HashMap<String, Value>);

impl SysctlConf {
    const FIRST_INDEX: usize = 0;
    const SINGLE_KEY_LENGTH: usize = 1;

    pub fn new(lines: Vec<String>) -> Self {
        let mut map = HashMap::new();

        for line in lines {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let keys: Vec<&str> = key.trim().split('.').collect();
                let value = value.trim();

                Self::insert_into_map(&mut map, &keys, value.to_string());
            }
        }

        Self(map)
    }

    fn insert_into_map(map: &mut HashMap<String, Value>, keys: &[&str], value: String) {
        if keys.is_empty() {
            return;
        }

        let key = keys[Self::FIRST_INDEX].to_string();
        if keys.len() == Self::SINGLE_KEY_LENGTH {
            map.insert(key, Value::String(value));
        } else {
            let entry = map.entry(key).or_insert_with(|| Value::Map(HashMap::new()));
            if let Value::Map(sub_map) = entry {
                Self::insert_into_map(sub_map, &keys[Self::FIRST_INDEX + 1..], value);
            }
        }
    }
}
