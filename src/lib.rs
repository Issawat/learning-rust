use std::{env, error::Error, fs};

pub struct Config {
    query_string: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Required arguments are missing!");
        }

        let query_string = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query_string,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let search_result: Vec<&str>;

    if config.ignore_case {
        search_result = search_ignore_case(&config.query_string, &contents)?;
    } else {
        search_result = search(&config.query_string, &contents)?;
    }

    for result in search_result {
        println!("{result}")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(Ok(vec!["safe, fast, productive."]), search(query, contents));
    }
    #[test]
    fn failure() {
        let query = "duvt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            Err("Couldn't find the word in the target file"),
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitve() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            Ok(vec!["safe, fast, productive."]),
            search_ignore_case(query, contents)
        );
    }
    #[test]
    fn failure_case_insensitve() {
        let query = "DuVt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            Err("Couldn't find the word in the target file"),
            search_ignore_case(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, &'static str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        return Err("Couldn't find the word in the target file");
    }

    Ok(results)
}

pub fn search_ignore_case<'a>(
    query: &str,
    contents: &'a str,
) -> Result<Vec<&'a str>, &'static str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        return Err("Couldn't find the word in the target file (non-case sensitive)");
    }

    Ok(results)
}
