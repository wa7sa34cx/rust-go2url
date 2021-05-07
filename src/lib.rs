use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn go(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", "Go Forest, go!");
    
    validate(&config.filename)?;

    Ok(())
}

fn validate(filename: &str) -> Result<(), &str> {
    if filename != "urls.txt" {
        return Err("not valid filename");
    }

    Ok(())
}