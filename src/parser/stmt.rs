use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_stmt_func_decl(&mut self) {
        fn action(ast: &mut Ast) {
            let stmts = ast.node_stack.pop().unwrap();
            let var_list = ast.node_stack.pop().unwrap();
            let mut var = ast.node_stack.pop().unwrap();

            for child in stmts.children {
                var.children.push(child);
            }

            var.children.push(var_list);
            var.token = Tok::FuncDecl;
            ast.node_stack.push(var);
        }

        self.install_prod(
            Tok::Stmt,
            &vec![
                Tok::FnKW,
                Tok::Var,
                Tok::LeftParen,
                Tok::VarList,
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

        self.install_prod(
            Tok::Stmt, 
            &vec![Tok::Var, Tok::Eq, Tok::Expr], 
            Some(action));
    }

    pub fn install_stmt_call(&mut self) {
        self.install_prod(
            Tok::Stmt, 
            &vec![Tok::FuncCall], 
            None);
    }

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

    pub fn install_stmt_if(&mut self) {
        self.install_prod(
            Tok::Stmt, 
            &vec![Tok::IfKW, Tok::Expr, Tok::Block], 
            None);
    }
}
