use crate::parser::Parser;
use crate::ast::{Ast, Node};
use crate::tokens::Tok;

impl Parser {
    pub fn install_expr_list_comma(&mut self) {
        self.install_prod(Tok::ExprList, &vec![
            Tok::Expr,
            Tok::Comma,
            Tok::ExprList],
            None);
    }

    pub fn install_expr_list_last(&mut self) {
        self.install_prod(Tok::ExprList, &vec![Tok::Expr], None);
    }

    pub fn install_expr_list_empty(&mut self) {
        fn action(ast: &mut Ast) {
            ast.push_node(Tok::ExprList, None);
        }

        self.install_prod(Tok::ExprList, &vec![], Some(action));
    }
}