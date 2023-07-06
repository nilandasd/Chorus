use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    /*
    pub fn install_stmt_return(&mut self) {
        fn action(ast: &mut Ast) {
            let expr = ast.node_stack.pop().unwrap();
            let mut return_kw = ast.new_node(Tok::ReturnKW, None);
            return_kw.children.push(expr);

            ast.node_stack.push(return_kw);
        }

        self.install_prod(
            Tok::Stmt, 
            &vec![Tok::ReturnKW, Tok::Expr], 
            Some(action));
    }
    */

    pub fn install_stmt_decl(&mut self) {
        self.install_prod(Tok::Stmt, &vec![Tok::Decl], None);
    }

    pub fn install_stmt_control(&mut self) {
        self.install_prod(Tok::Stmt, &vec![Tok::Control], None);
    }

    pub fn install_stmt_expr(&mut self) {
        self.install_prod(Tok::Stmt, &vec![Tok::Expr, Tok::SemiColon], None);
    }
}
