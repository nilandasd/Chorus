use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

// TODO add if, ifelse, while, for in

impl Parser {
    pub fn install_stmt_func_decl(&mut self) {
        fn action(ast: &mut Ast) {
            let mut stmts = ast.node_stack.pop().unwrap();
            let _ = ast.node_stack.pop().unwrap();

            stmts.token = Tok::FuncDecl;
            ast.node_stack.push(stmts);
            println!("reducing func decl");
        }

        self.install_prod(
            Tok::Stmt,
            &vec![
                Tok::FnKW,
                Tok::Var,
                Tok::LeftParen,
                Tok::ArgList,
                Tok::RightParen,
                Tok::Block,
            ],
            Some(action),
        );
    }

    pub fn install_stmt_var_decl(&mut self) {
        fn action(ast: &mut Ast) {
            let expr = ast.node_stack.pop().unwrap();
            let mut eq = ast.node_stack.pop().unwrap();
            let var = ast.node_stack.pop().unwrap();

            eq.val = var.val;
            eq.children.push(expr);

            ast.node_stack.push(eq);
        }

        self.install_prod(Tok::Stmt, &vec![Tok::Var, Tok::Eq, Tok::Expr], Some(action));
    }

    pub fn install_stmt_expr(&mut self) {
        self.install_prod(Tok::Stmt, &vec![Tok::Expr], None);
    }

    pub fn install_stmt_if(&mut self) {
        self.install_prod(Tok::Stmt, &vec![Tok::IfKW, Tok::Expr, Tok::Block], None);
    }
}
