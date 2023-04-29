use crate::ast::Ast;
use crate::generator::Generator;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

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
        self.lexer.open_file(file_path);
        self.parser.build_ast(&mut self.lexer, &mut self.ast);
        self.ast.display();
        self.ast.traverse(&mut self.generator);
        self.generator.gen_end();
        self.generator.display();
        self.interpreter.run(&mut self.generator);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/hello_world.ch");
    }

    #[test]
    fn basic() {
        let mut chorus = Chorus::init();
        chorus.interpret("examples/basic.ch");
    }
}
