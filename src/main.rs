use std::{env, process};

use minigrep::Cfg;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Cfg::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(cfg) {
        println!("Problem parsing arguments: {e}");
        process::exit(1)
    }
}
