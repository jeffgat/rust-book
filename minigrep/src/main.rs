// minigrep should take in two args, file path and string to search for
// ex. cargo run -- searchstring example-filename.txt
// -- ends the options and everything after will be treated as args

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // std::env::args() will accept unicode args from the CLI

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing args: {err}");
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("application errior: {e}");
        process::exit(1)
    }
}

