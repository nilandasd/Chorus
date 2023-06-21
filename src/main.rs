mod ast;
mod app;
mod config;
mod generator;
mod lexer;
mod parser;
mod tokens;
mod block;
mod bump_block;
mod constants;
mod raw_ptr;
mod allocator;
mod heap;
mod header;

use crate::app::App;
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
    let mut app = App::init();

    app.run(config.filename);
}
