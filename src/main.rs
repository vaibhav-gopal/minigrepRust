//Run the program via `cargo run -- examplesearchstring example-filename.txt
//  the `--` indicates that the following arguments are for the program rather than cargo

//help us read command-line arguments
use std::env;
//exit program
use std::process;
//bring in library crate into scope
use minigrep::{Config, run};

fn main() {
    //notice that the first argument is always the name of the binary --> we don't need it in this program
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //print to stderr rather than stdout with eprintln! macro for error messages
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}