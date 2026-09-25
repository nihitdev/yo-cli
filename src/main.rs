mod app;
mod args;
mod completions;
mod config;
mod content;
mod doctor;
mod editor;
mod fetch;
mod git;
mod process;
mod project;
mod snapshot;
mod timer;
mod tips;
mod ui;

use std::{env, process::exit};

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let command = match args::parse(&raw_args) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("error: {error}\n\n{}", args::help_text());
            exit(2);
        }
    };

    if let Err(error) = app::execute(command) {
        eprintln!("yoo: {error}");
        exit(1);
    }
}
