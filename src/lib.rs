use colored::*;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        let grab = grab_match(line, &config.query, config.case_sensitive).unwrap();

        println!(
            "{}{}{}",
            &line[..grab.first_index],
            &line[grab.first_index..grab.last_index].green(),
            &line[grab.last_index..],
        );
    }

    Ok(())
}

struct Match {
    first_index: usize,
    last_index: usize,
}

fn grab_match<'a>(text: &'a str, query: &str, case_sensitive: bool) -> Result<Match, &'a str> {
    let first_index;

    if case_sensitive {
        first_index = text.find(query).unwrap();
    } else {
        first_index = text.to_lowercase().find(&query.to_lowercase()).unwrap();
    }

    let last_index = first_index + query.len();

    Ok(Match {
        first_index,
        last_index,
    })
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn match_case() {
        let query = "rUsT";
        let line = "Rust is a safe and productive language";

        let grab = grab_match(line, query, false).unwrap();

        assert_eq!(0, grab.first_index);
    }

    #[test]
    #[should_panic]
    fn match_no_case() {
        let query = "rUsT";
        let line = "Rust is a safe and productive language";

        let grab = grab_match(line, query, true).unwrap();

        assert_eq!(0, grab.first_index);
    }
}
