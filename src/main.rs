use std::{env, process};

use ripripgrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("[ERROR] {e}");
        process::exit(1)
    }
}