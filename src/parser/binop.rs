use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_binop_plus(&mut self) {
        self.install_prod(Tok::BinOp, &vec![Tok::Plus], None);
    }
}