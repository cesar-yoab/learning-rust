use clap::Parser;
use std::process;

#[derive(Parser)]
#[clap(name = "NotGrep")]
#[clap(version = "0.0.1")]
#[clap(about="A small and Rust implemented version of grep - print lines that match patterns", long_about = None)]
struct Args {
    /// Pattern to search.
    pattern: String,

    /// Filename.
    filename: String,
    /// Interpret PATTERN as extended regular expressions.
    #[clap(short, long)]
    extended_regexp: bool,

    /// Interpret PATTERN as fixed strings, not regular expressions. This is the default.
    #[clap(short, long)]
    fixed_strings: bool,

    /// Interpret PATTERN as basic regular expressions.
    #[clap(short, long)]
    basic_regexp: bool,

    /// Perform search with case sensitivity on.
    #[clap(long)]
    case_sensitive: bool,

    /// Suppress normal output; instead print a count of matching lines for each input file.
    #[clap(short, long)]
    count: bool,

    /// Quiet; do not write anything to standard output. Exit immediately with zero status if any match is found.
    #[clap(short, long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    // Check that flags are used properly
    if args.extended_regexp && args.fixed_strings {
        eprintln!("You can only use one of --extended-regexp or --fixed-strings.");
        process::exit(1);
    } else if args.extended_regexp && args.basic_regexp || args.fixed_strings && args.basic_regexp {
        eprintln!("You cannot use --basic-regexp with --extended-regexp or --fixed-strings.");
        process::exit(1);
    }

    let config = notgrep::Config::new(
        args.pattern.clone(),
        args.filename.clone(),
        args.case_sensitive,
        args.quiet,
        args.count,
    );
    if args.extended_regexp {
        if let Err(e) = notgrep::run_extended(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    } else if args.basic_regexp {
        if let Err(e) = notgrep::run_basic(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = notgrep::run_fixed_str(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
