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
    const KEY_INDEX: usize = 1;
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
