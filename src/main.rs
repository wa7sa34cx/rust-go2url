use std::env;
use std::process;

use go2url::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("{:?}", config.filename);

    if let Err(e) = go2url::go(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
