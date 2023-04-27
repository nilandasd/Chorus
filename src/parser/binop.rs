use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_binops(&mut self) {
        self.install_prod(Tok::BinOp, &vec![Tok::Plus], None);
        self.install_prod(Tok::BinOp, &vec![Tok::Minus], None);
    }
}