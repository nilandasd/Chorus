use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::ast::Ast;

pub struct Chorus {
    lexer: Lexer,
    parser: Parser,
    ast: Ast,
}

impl Chorus {
    pub fn init() -> Self {
        Self {
            lexer: Lexer::init(),
            parser: Parser::init(),
            ast: Ast::init(),
        }
    }

    pub fn compile(&mut self, file_path: &str) {
        self.lexer.open_file(file_path);
        self.parser.build_ast(&mut self.ast, &mut self.lexer);
        self.ast.display();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut chorus = Chorus::init();
        chorus.compile("examples/hello_world.ch");
    }
}