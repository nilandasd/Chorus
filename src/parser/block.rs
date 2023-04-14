use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_block(&mut self) {
        self.install_prod(Tok::Block, &vec![
            Tok::LeftCurly,
            Tok::Stmts,
            Tok::RightCurly], None);
    }
}