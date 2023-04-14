use crate::parser::Parser;
use crate::ast::{Ast, Node};
use crate::tokens::Tok;

impl Parser {
    pub fn install_start(&mut self) {
        self.install_prod(Tok::Start, &vec![
            Tok::Stmts,
            Tok::End],
        None);
    }

    pub fn install_stmts_list(&mut self) {
        fn action(ast: &mut Ast) {
            let mut stmts = ast.node_stack.pop().unwrap();
            let stmt = ast.node_stack.pop().unwrap();

            stmts.children.push(stmt);
            ast.node_stack.push(stmts);
            //println!("stmts list reduct");
        }

        self.install_prod(Tok::Stmts, &vec![
            Tok::Stmts,
            Tok::Stmt],
        Some(action));
    }

    pub fn install_stmts_last(&mut self) {
        fn action(ast: &mut Ast) {
            let stmts = Node {
                token: Tok::Stmts,
                type_id: None,
                val: None,
                children: vec![]
            };

            ast.node_stack.push(stmts);
            //println!("stmts last reduct");
        }

        self.install_prod(Tok::Stmts, &vec![], Some(action));
    }
}