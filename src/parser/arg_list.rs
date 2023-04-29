use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_arg_list_comma(&mut self) {
        self.install_prod(
            Tok::ArgList,
            &vec![Tok::Var, Tok::Comma, Tok::ArgList],
            None,
        );
    }

    pub fn install_arg_list_last(&mut self) {
        self.install_prod(Tok::ArgList, &vec![Tok::Var], None);
    }

    pub fn install_arg_list_empty(&mut self) {
        self.install_prod(
            Tok::ArgList, 
            &vec![], 
            None);
    }
}
