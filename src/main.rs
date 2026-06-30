mod app;
mod args;
mod config;
mod content;
mod doctor;
mod fetch;
mod git;
mod project;
mod timer;
mod tips;
mod ui;

use std::{env, process};

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let command = match args::parse(&raw_args) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("error: {error}\n\n{}", args::help_text());
            process::exit(2);
        }
    };

    if let Err(error) = app::execute(command) {
        eprintln!("yoo: {error}");
        process::exit(1);
    }
}
