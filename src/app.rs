use crate::ast::Ast;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::generator::Generator;
use crate::heap::Heap;

use std::env;

pub struct App {
    lexer: Lexer,
    parser: Parser,
    ast: Ast,
    generator: Generator,
    //heap: Heap,
    // interpreter: Interpreter,
}

impl App {
    pub fn init() -> Self {
        Self {
            lexer: Lexer::init(),
            parser: Parser::init(),
            ast: Ast::init(),
            generator: Generator::init(),
            //heap: Heap::default()
        }
    }

    pub fn run(&mut self, file_path: &str) {
        if self.lexer.open_file(file_path).is_err() { return; }

        self.parser.build_ast(&mut self.lexer, &mut self.ast);
        self.ast.traverse(&mut self.generator);
        // generator.optimize();

        if env::var("DEBUG").is_ok() {
            self.ast.display();
        }

        // if errors
        //   display lex errors
        //   display ast errors

        self.ast.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_words() {
        let mut app = App::init();
        app.run("examples/first_words.ch");
    }

    #[test]
    fn hello_world() {
        let mut app = App::init();
        app.run("examples/hello_world.ch");
    }

    #[test]
    fn foobar() {
        let mut app = App::init();
        app.run("examples/foobar.ch");
    }

    #[test]
    fn numbers() {
        let mut app = App::init();
        app.run("examples/numbers.ch");
    }
}
