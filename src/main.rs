mod allocator;
mod app;
mod arena;
mod array;
mod ast;
mod block;
mod bump_block;
mod bytecode;
mod config;
mod constants;
mod container;
mod error;
mod dict;
mod function;
mod generator;
mod hashable;
mod heap;
mod header;
mod lexer;
mod list;
mod memory;
mod number;
mod parser;
mod pair;
mod ptr_ops;
mod printer;
mod raw_ptr;
mod raw_array;
mod safe_ptr;
mod symbol_map;
mod symbol;
mod tagged_ptr;
mod text;
mod tokens;
mod vm;

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
