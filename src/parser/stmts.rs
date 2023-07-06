use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_start(&mut self) {
        self.install_prod(Tok::Start, &vec![Tok::Stmts, Tok::End], None);
    }

    pub fn install_stmts_list(&mut self) {
        fn action(ast: &mut Ast) {
            let mut stmts = ast.node_stack.pop().unwrap();
            let stmt = ast.node_stack.pop().unwrap();

            stmts.children.push(stmt);
            ast.node_stack.push(stmts);
        }

        self.install_prod(Tok::Stmts, &vec![Tok::Stmt, Tok::Stmts], Some(action));
    }

    pub fn install_stmts_empty(&mut self) {
        fn action(ast: &mut Ast) {
            ast.push_node(Tok::Stmts, None);
        }

        self.install_prod(Tok::Stmts, &vec![], Some(action));
    }
}
