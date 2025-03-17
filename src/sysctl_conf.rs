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

        let key = keys[0].to_string();
        if keys.len() == 1 {
            map.insert(key, Value::String(value));
        } else {
            let entry = map.entry(key).or_insert_with(|| Value::Map(HashMap::new()));
            if let Value::Map(sub_map) = entry {
                Self::insert_into_map(sub_map, &keys[1..], value);
            }
        }
    }
}
