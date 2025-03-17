use sysctl_conf_parser::parse_sysctl_from_path;
use thiserror::Error;

const EXPECTED_ARG_COUNT: usize = 3;
const PROGRAM_NAME_INDEX: usize = 0;
const SYSCTL_CONF_FILE_PATH_ARG_INDEX: usize = 1;
const SCHEMA_FILE_PATH_ARG_INDEX: usize = 2;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Usage: {0} <path_to_sysctl.conf> <path_to_schema>")]
    InvalidArguments(String),
    #[error("Error parsing config file: {0}")]
    ParseError(String),
}

fn main() -> Result<(), ConfigError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != EXPECTED_ARG_COUNT {
        return Err(ConfigError::InvalidArguments(
            args[PROGRAM_NAME_INDEX].clone(),
        ));
    }
    let sysctl_conf_file_path = &args[SYSCTL_CONF_FILE_PATH_ARG_INDEX];
    let schema_file_path = &args[SCHEMA_FILE_PATH_ARG_INDEX];

    match parse_sysctl_from_path(sysctl_conf_file_path, &schema_file_path) {
        Ok(config) => {
            println!("Parsed config: {:?}", config);
            Ok(())
        }
        Err(e) => Err(ConfigError::ParseError(e.to_string())),
    }
}
