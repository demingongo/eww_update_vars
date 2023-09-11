use eww_update_vars::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // run application
    if let Err(e) = eww_update_vars::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
