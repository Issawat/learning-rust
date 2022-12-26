use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text content:\n {contents}");
    Ok(())
}

struct Config {
    query_string: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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
