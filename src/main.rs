mod config;
mod tokens;
mod lexer;
mod parser;
mod ast;
mod generator;
mod interpreter;
mod chorus;

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