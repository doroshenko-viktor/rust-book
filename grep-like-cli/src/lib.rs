use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("searching for {}", config.query);
    println!("searching in file {}", config.filename);

    let contents = fs::read_to_string(&config.filename)?;
    let lines_found = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Lines found:");
    for line in lines_found {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get query string"),
        };

        let filename = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

//     let query = query.to_lowercase();
//     contents
//         .lines()
//         .filter(|line| line.to_lowercase().contains(query))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive
        and something else.
        Duct
        ";
        assert_eq!(
            vec!["        safe, fast, productive"],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
        Rust:
        safe, fast, productive
        and something else.
        ";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents))
    }
}
