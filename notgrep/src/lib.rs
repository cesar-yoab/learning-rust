use std::error::Error;
use std::fs;

pub struct Config {
  pattern: String,
  filename: String,
  case_sensitive: bool,
  count: bool,
  quiet: bool,
}

impl Config {
  pub fn new(
    pattern: String,
    filename: String,
    case_sensitive: bool,
    count: bool,
    quiet: bool,
  ) -> Config {
    Config {
      pattern,
      filename,
      case_sensitive,
      count,
      quiet,
    }
  }
}

pub fn run_basic(config: Config) -> Result<(), Box<dyn Error>> {
  Ok(())
}

pub fn run_extended(config: Config) -> Result<(), Box<dyn Error>> {
  Ok(())
}

pub fn run_fixed_str(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    fixed_search(&config.pattern, &contents)
  } else {
    fixed_search_insensitive(&config.pattern, &contents)
  };

  if !config.quiet {
    if config.count {
      println!("{} matches found.", results.len());
    } else {
      for line in results {
        println!("{}", line);
      }
    }
  }

  Ok(())
}

fn fixed_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(pattern) {
      results.push(line);
    }
  }

  results
}

fn fixed_search_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let pattern = pattern.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&pattern) {
      results.push(line);
    }
  }

  results
}
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["safe, fast, productive."],
      fixed_search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      fixed_search_insensitive(query, content)
    );
  }
}
