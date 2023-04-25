use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::generator::Generator;
pub struct Chorus {
    lexer: Lexer,
    parser: Parser,
    generator: Generator,

}

impl Chorus {
    pub fn init() -> Self {
        Self {
            lexer: Lexer::init(),
            parser: Parser::init(),
            generator: Generator::init(),
        }
    }

    pub fn interpret(&mut self, file_path: &str) {
        self.lexer.open_file(file_path);
        self.parser.build_ast(&mut self.lexer, &mut self.generator);
        self.parser.display_ast();
        // self.parser.display_scope();
        // convert to linear IR
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