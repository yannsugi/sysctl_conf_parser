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
            } else {
                let keys: Vec<&str> = line.split('.').collect();
                Self::insert_into_map(&mut map, &keys, String::new());
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

    pub fn get(&self, key: &str) -> Option<&String> {
        let keys: Vec<&str> = key.split('.').collect();
        Self::get_from_map(&self.0, &keys)
    }

    fn get_from_map<'a>(map: &'a HashMap<String, Value>, keys: &[&str]) -> Option<&'a String> {
        if keys.is_empty() {
            return None;
        }

        let key = keys[Self::FIRST_INDEX];
        if let Some(value) = map.get(key) {
            if keys.len() == Self::SINGLE_KEY_LENGTH {
                if let Value::String(ref s) = value {
                    Some(s)
                } else {
                    None
                }
            } else if let Value::Map(sub_map) = value {
                Self::get_from_map(sub_map, &keys[Self::FIRST_INDEX + 1..])
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_conf_new() {
        let lines = vec![
            "net.ipv4.ip_forward=1".to_string(),
            "net.ipv4.conf.all.rp_filter=1".to_string(),
            "kernel.hostname".to_string(),
            "vm.swappiness=".to_string(),
            "".to_string(),
            "endpoint = localhost::3000".to_string(),
            "# This is a comment".to_string(),
        ];

        let sysctl_conf = SysctlConf::new(lines);

        let mut expected_data = HashMap::new();

        let mut ipv4_map = HashMap::new();
        let mut conf_map = HashMap::new();
        let mut all_map = HashMap::new();
        let mut net_map = HashMap::new();
        let mut kernel_map = HashMap::new();
        let mut vm_map = HashMap::new();

        ipv4_map.insert("ip_forward".to_string(), Value::String("1".to_string()));
        conf_map.insert("rp_filter".to_string(), Value::String("1".to_string()));
        all_map.insert("all".to_string(), Value::Map(conf_map));
        ipv4_map.insert("conf".to_string(), Value::Map(all_map));
        net_map.insert("ipv4".to_string(), Value::Map(ipv4_map));
        kernel_map.insert("hostname".to_string(), Value::String(String::new()));
        vm_map.insert("swappiness".to_string(), Value::String(String::new()));
        expected_data.insert("net".to_string(), Value::Map(net_map));
        expected_data.insert("kernel".to_string(), Value::Map(kernel_map));
        expected_data.insert("vm".to_string(), Value::Map(vm_map));
        expected_data.insert(
            "endpoint".to_string(),
            Value::String("localhost::3000".to_string()),
        );

        assert_eq!(sysctl_conf.0, expected_data);
    }

    #[test]
    fn test_sysctl_conf_get() {
        let lines = vec![
            "net.ipv4.ip_forward=1".to_string(),
            "net.ipv4.conf.all.rp_filter=1".to_string(),
            "kernel.hostname".to_string(),
            "vm.swappiness=".to_string(),
            "endpoint = localhost::3000".to_string(),
        ];

        let sysctl_conf = SysctlConf::new(lines);

        assert_eq!(
            sysctl_conf.get("net.ipv4.ip_forward"),
            Some(&"1".to_string())
        );
        assert_eq!(
            sysctl_conf.get("net.ipv4.conf.all.rp_filter"),
            Some(&"1".to_string())
        );
        assert_eq!(sysctl_conf.get("kernel.hostname"), Some(&String::new()));
        assert_eq!(sysctl_conf.get("vm.swappiness"), Some(&String::new()));
        assert_eq!(
            sysctl_conf.get("endpoint"),
            Some(&"localhost::3000".to_string())
        );
        assert_eq!(sysctl_conf.get("non.existent.key"), None);
    }
}
