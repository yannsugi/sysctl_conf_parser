use std::collections::HashMap;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SysctlConfSchemaError {
    #[error("Invalid value type for key '{0}'")]
    InvalidValueType(String),
    #[error("Key '{0}' is not defined in the schema")]
    UndefinedKey(String),
}

#[derive(Debug, PartialEq)]
pub enum SysctlConfSchemaValueType {
    String,
    Bool,
    Integer,
    Float,
}

impl TryFrom<&str> for SysctlConfSchemaValueType {
    type Error = SysctlConfSchemaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "string" => Ok(SysctlConfSchemaValueType::String),
            "bool" => Ok(SysctlConfSchemaValueType::Bool),
            "integer" => Ok(SysctlConfSchemaValueType::Integer),
            "float" => Ok(SysctlConfSchemaValueType::Float),
            _ => Err(SysctlConfSchemaError::InvalidValueType(value.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct SysctlConfSchema(HashMap<String, SysctlConfSchemaValueType>);

impl SysctlConfSchema {
    const SCHEMA_PARTS_LEN: usize = 2;
    const KEY_INDEX: usize = 0;
    const VALUE_TYPE_INDEX: usize = 1;

    pub fn new(schema_line_list: Vec<String>) -> Result<Self, SysctlConfSchemaError> {
        let map = schema_line_list
            .into_iter()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
                if parts.len() == Self::SCHEMA_PARTS_LEN {
                    Some((
                        parts[Self::KEY_INDEX].to_string(),
                        parts[Self::VALUE_TYPE_INDEX].to_string(),
                    ))
                } else {
                    None
                }
            })
            .into_iter()
            .map(|(key, value_type)| {
                Ok((
                    key,
                    SysctlConfSchemaValueType::try_from(value_type.as_str())?,
                ))
            })
            .collect::<Result<HashMap<String, SysctlConfSchemaValueType>, SysctlConfSchemaError>>(
            )?;

        Ok(Self(map))
    }

    pub fn validate_value_type(&self, key: &str, value: &str) -> Result<(), SysctlConfSchemaError> {
        match self.0.get(key) {
            Some(SysctlConfSchemaValueType::String) => Ok(()),
            Some(SysctlConfSchemaValueType::Bool) => {
                if value == "true" || value == "false" {
                    Ok(())
                } else {
                    Err(SysctlConfSchemaError::InvalidValueType(key.to_string()))
                }
            }
            Some(SysctlConfSchemaValueType::Integer) => {
                if value.parse::<i64>().is_ok() {
                    Ok(())
                } else {
                    Err(SysctlConfSchemaError::InvalidValueType(key.to_string()))
                }
            }
            Some(SysctlConfSchemaValueType::Float) => {
                if value.parse::<f64>().is_ok() {
                    Ok(())
                } else {
                    Err(SysctlConfSchemaError::InvalidValueType(key.to_string()))
                }
            }
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_conf_schema_new() {
        let lines = vec![
            "net.ipv4.ip_forward -> integer".to_string(),
            "net.ipv4.conf.all.rp_filter -> bool".to_string(),
            "kernel.hostname -> string".to_string(),
            "vm.swappiness -> float".to_string(),
            "endpoint -> string".to_string(),
        ];

        let schema = SysctlConfSchema::new(lines).unwrap();

        let mut expected_data = HashMap::new();
        expected_data.insert(
            "net.ipv4.ip_forward".to_string(),
            SysctlConfSchemaValueType::Integer,
        );
        expected_data.insert(
            "net.ipv4.conf.all.rp_filter".to_string(),
            SysctlConfSchemaValueType::Bool,
        );
        expected_data.insert(
            "kernel.hostname".to_string(),
            SysctlConfSchemaValueType::String,
        );
        expected_data.insert(
            "vm.swappiness".to_string(),
            SysctlConfSchemaValueType::Float,
        );
        expected_data.insert("endpoint".to_string(), SysctlConfSchemaValueType::String);

        assert_eq!(schema.0, expected_data);
    }

    #[test]
    fn test_sysctl_conf_schema_validate_value_type() {
        let lines = vec![
            "net.ipv4.ip_forward -> integer".to_string(),
            "net.ipv4.conf.all.rp_filter -> bool".to_string(),
            "kernel.hostname -> string".to_string(),
            "vm.swappiness -> float".to_string(),
            "endpoint -> string".to_string(),
        ];

        let schema = SysctlConfSchema::new(lines).unwrap();

        assert!(schema
            .validate_value_type("net.ipv4.ip_forward", "123")
            .is_ok());
        assert!(schema
            .validate_value_type("net.ipv4.conf.all.rp_filter", "true")
            .is_ok());
        assert!(schema
            .validate_value_type("kernel.hostname", "localhost")
            .is_ok());
        assert!(schema.validate_value_type("vm.swappiness", "0.5").is_ok());
        assert!(schema
            .validate_value_type("endpoint", "localhost::3000")
            .is_ok());

        assert!(schema
            .validate_value_type("net.ipv4.ip_forward", "abc")
            .is_err());
        assert!(schema
            .validate_value_type("net.ipv4.conf.all.rp_filter", "yes")
            .is_err());
        assert!(schema.validate_value_type("vm.swappiness", "abc").is_err());
    }

    #[test]
    fn test_sysctl_conf_schema_try_from_failure() {
        assert!(SysctlConfSchemaValueType::try_from("unknown").is_err());
        assert!(SysctlConfSchemaValueType::try_from("123").is_err());
        assert!(SysctlConfSchemaValueType::try_from("").is_err());
    }

    #[test]
    fn test_sysctl_conf_schema_try_from_case_insensitive() {
        assert_eq!(
            SysctlConfSchemaValueType::try_from("STRING").unwrap(),
            SysctlConfSchemaValueType::String
        );
        assert_eq!(
            SysctlConfSchemaValueType::try_from("BOOL").unwrap(),
            SysctlConfSchemaValueType::Bool
        );
        assert_eq!(
            SysctlConfSchemaValueType::try_from("INTEGER").unwrap(),
            SysctlConfSchemaValueType::Integer
        );
        assert_eq!(
            SysctlConfSchemaValueType::try_from("FLOAT").unwrap(),
            SysctlConfSchemaValueType::Float
        );
    }
}
