use std::{env, process};

use ripripgrep::{run, Config};

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("[ERROR] Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("[ERROR] {e}");
        process::exit(1)
    }
}
