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
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    basic_search(&config.pattern, &contents)
  } else {
    basic_search_insensitive(&config.pattern, &contents)
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

fn basic_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(pattern) {
      results.push(line);
    }
  }

  results
}

fn basic_search_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let pattern_lower = pattern.to_lowercase();

  for line in contents.lines() {
    if line.contains(pattern) {
      results.push(line);
    }

    if line.contains(&pattern_lower) {
      results.push(line);
    }
  }

  results
}

pub fn run_extended(config: Config) -> Result<(), Box<dyn Error>> {
  Ok(())
}

pub fn run_fixed_str(config: Config) -> Result<(), Box<dyn Error>> {
  Ok(())
}
