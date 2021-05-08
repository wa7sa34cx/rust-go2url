use dotenv::dotenv;
use rand::seq::SliceRandom;
use std::env;
use std::error::Error;
use std::fs;

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

    // Create path to file
    let path = create_path(&config.filename)?;

    // Get one line from file
    let url = get_rand_line(&path)?;

    println!("{:?}", url);

    Ok(())
}

fn validate(filename: &str) -> Result<(), &str> {
    // if filename != "urls.txt" {
    //     return Err("not valid filename");
    // }

    Ok(())
}

fn create_path(filename: &str) -> Result<String, &str> {
    dotenv().ok();

    match env::var("PATH_TO_GO_FOLDER") {
        Err(e) => Err(e).unwrap(),
        Ok(val) => Ok(val + filename),
    }
}

fn get_rand_line(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let urls: Vec<&str> = contents.split('\n').collect();
    let url = urls.choose(&mut rand::thread_rng());

    match url {
        None => Err("Error with rand".into()),
        Some(item) => Ok(item.clone().to_string()),
    }
}
