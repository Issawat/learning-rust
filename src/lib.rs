use std::{error::Error, fs};

pub struct Config {
    query_string: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Required arguments are missing!");
        }

        let query_string = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query_string,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text content:\n {contents}");
    Ok(())
}
