use dotenv::dotenv;
use rand::seq::SliceRandom;
use std::env;
use std::env::VarError;
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

    // Get contents
    let contents = fs::read_to_string(path)?;

    // Get one line from contents
    let line = get_rand_line(&contents)?;

    println!("{:?}", line);

    Ok(())
}

fn validate(filename: &str) -> Result<(), &str> {
    // if filename != "urls.txt" {
    //     return Err("not valid filename");
    // }

    Ok(())
}

fn create_path(filename: &str) -> Result<String, VarError> {
    dotenv().ok();

    match env::var("PATH_TO_GO_FOLDER") {
        Err(e) => Err(e),
        Ok(val) => Ok(val + filename),
    }
}

fn get_rand_line(contents: &str) -> Result<String, &str> {
    let urls: Vec<&str> = contents.split('\n').collect();
    let url = urls.choose(&mut rand::thread_rng());

    let url = match url {
        None => return Err("Error with rand"),
        Some(item) => item.clone().to_string(),
    };

    return if url.is_empty() {
        Err("File is empty")
    } else {
        Ok(url)
    };
}
