use sysctl_conf_parser::parse_sysctl_from_path;

const EXPECTED_ARG_COUNT: usize = 2;
const ERROR_EXIT_CODE: i32 = 1;
const PROGRAM_NAME_INDEX: usize = 0;
const FILEPATH_ARG_INDEX: usize = 1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != EXPECTED_ARG_COUNT {
        eprintln!("Usage: {} <path_to_sysctl.conf>", args[PROGRAM_NAME_INDEX]);
        std::process::exit(ERROR_EXIT_CODE);
    }
    let filename = &args[FILEPATH_ARG_INDEX];

    match parse_sysctl_from_path(filename) {
        Ok(config) => println!("Parsed config: {:?}", config),
        Err(e) => eprintln!("Error parsing config file: {}", e),
    }
}
