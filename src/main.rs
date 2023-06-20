mod ast;
mod chorus;
mod config;
mod generator;
mod lexer;
mod parser;
mod tokens;

use crate::chorus::Chorus;
use crate::config::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(&config);
}

fn run(config: &Config) {
    let mut chorus = Chorus::init();

    chorus.interpret(config.filename);
}
