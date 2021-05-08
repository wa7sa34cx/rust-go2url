use std::error::Error;
use std::fs;
use rand::seq::SliceRandom;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn go(config: Config) -> Result<(), Box<dyn Error>> {    
    // validate filname
    validate(&config.filename)?;

    // get one line from file
    let url = get_rand_line(&config.filename)?;

    println!("{:?}", url);

    Ok(())
}

fn validate(filename: &str) -> Result<(), &str> {
    // if filename != "urls.txt" {
    //     return Err("not valid filename");
    // }

    Ok(())
}

fn get_rand_line(filename: &str) -> Result<String, Box<dyn Error>> {    
    let path = format!("./go/{}", filename);
    let contents = fs::read_to_string(path)?;
    
    let urls: Vec<&str> = contents.split('\n').collect();
    let url = urls.choose(&mut rand::thread_rng());

    match url {
        None => Err("File is empty".into()),
        Some(item) => Ok(item.clone().to_string()),
    }
}