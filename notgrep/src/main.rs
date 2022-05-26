use clap::Parser;

#[derive(Parser)]
#[clap(name = "NotGrep")]
#[clap(version = "0.0.1")]
#[clap(about="A small and Rust implemented version of grep - print lines that match patterns", long_about = None)]
struct Args {
    /// Filename.
    filename: String,

    /// Pattern to search.
    pattern: String,

    /// Interpret PATTERN as extended regular expressions.
    #[clap(short, long)]
    extended_regexp: bool,

    /// Interpret PATTERN as fixed strings, not regular expressions.
    #[clap(short, long)]
    fixed_strings: bool,

    /// Interpret PATTERN as basic regular expressions. This is the default.
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

    println!("Searching for pattern: {}", args.pattern);
    println!("On file: {}", args.filename);
}
