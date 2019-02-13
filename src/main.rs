use std::io::Read;
use std::env;
use std::fs;

mod config;
mod combine;
mod error;

const HELP: &'static str = r#"
No config file path provided.
$ cargo run -- path_to_config_file
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{}", HELP);
    }

    let config_file = &args[1];

	let mut file = fs::File::open(config_file)
        .expect("Failed to open config file");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Failed to read config file");

    let config = config::Config::new(&string)
        .expect("Parsing config file failed");

    combine::combine(config)
        .expect("Combining images failed");
}
