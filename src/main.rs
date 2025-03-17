use sysctl_conf_parser::parse_sysctl_from_path;

const EXPECTED_ARG_COUNT: usize = 3;
const ERROR_EXIT_CODE: i32 = 1;
const PROGRAM_NAME_INDEX: usize = 0;
const SYSCTL_CONF_FILE_PATH_ARG_INDEX: usize = 1;
const SCHEMA_FILE_PATH_ARG_INDEX: usize = 2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != EXPECTED_ARG_COUNT {
        eprintln!(
            "Usage: {} <path_to_sysctl.conf> <path_to_schema>",
            args[PROGRAM_NAME_INDEX]
        );
        std::process::exit(ERROR_EXIT_CODE);
    }
    let sysctl_conf_file_path = &args[SYSCTL_CONF_FILE_PATH_ARG_INDEX];
    let schema_file_path = &args[SCHEMA_FILE_PATH_ARG_INDEX];

    match parse_sysctl_from_path(sysctl_conf_file_path, &schema_file_path) {
        Ok(config) => println!("Parsed config: {:?}", config),
        Err(e) => {
            eprintln!("Error parsing config file: {}", e);
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}
