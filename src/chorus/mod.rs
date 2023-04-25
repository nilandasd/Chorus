use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::Ast;
use crate::generator::Generator;
pub struct Chorus {
    lexer: Lexer,
    parser: Parser,
    generator: Generator,
    ast: Ast,

}

impl Chorus {
    pub fn init() -> Self {
        Self {
            lexer: Lexer::init(),
            parser: Parser::init(),
            ast: Ast::init(),
            generator: Generator::init(),
        }
    }

    pub fn interpret(&mut self, file_path: &str) {
        self.lexer.open_file(file_path);
        self.parser.build_ast(&mut self.lexer, &mut self.ast);
        self.ast.traverse();
        self.ast.display();
        //self.interpreter.run(self.generator.code);
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