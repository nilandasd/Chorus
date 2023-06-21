mod ast;
mod app;
mod allocator;
mod block;
mod bump_block;
mod config;
mod constants;
mod generator;
mod heap;
mod header;
mod lexer;
mod memory;
mod parser;
mod ptr_ops;
mod raw_ptr;
mod safe_ptr;
mod tokens;
mod tagged_ptr;

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
