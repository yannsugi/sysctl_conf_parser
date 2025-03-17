use sysctl_conf_parser::parse_sysctl_from_path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_sysctl.conf>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    match parse_sysctl_from_path(filename) {
        Ok(config) => println!("Parsed config: {:?}", config),
        Err(e) => eprintln!("Error parsing config file: {}", e),
    }
}
