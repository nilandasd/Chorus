use crate::ast::Ast;
use crate::generator::Generator;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;

pub struct Chorus {
    lexer: Lexer,
    parser: Parser,
    ast: Ast,
    generator: Generator,
    interpreter: Interpreter,
}

impl Chorus {
    pub fn init() -> Self {
        Self {
            lexer: Lexer::init(),
            parser: Parser::init(),
            ast: Ast::init(),
            generator: Generator::init(),
            interpreter: Interpreter::init(),
        }
    }

    pub fn interpret(&mut self, file_path: &str) {
        if self.lexer.open_file(file_path).is_err() { return; }

        self.parser.build_ast(&mut self.lexer, &mut self.ast);
        self.ast.traverse(&mut self.generator);
        // generator.optimize();

        if env::var("DEBUG").is_ok() {
            self.ast.display();
            self.generator.display();
        }

        // if errors
        //   display lex errors
        //   display ast errors

        self.ast.clear();

        self.interpreter.run(&mut self.generator);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_words() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/first_words.ch");
    }

    #[test]
    fn hello_world() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/hello_world.ch");
    }

    #[test]
    fn foobar() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/foobar.ch");
    }

    #[test]
    fn numbers() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/numbers.ch");
    }
}
