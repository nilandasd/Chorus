use crate::parser::Parser;
use crate::ast::Ast;
use crate::tokens::Tok;

impl Parser {
    pub fn install_stmt_func_decl(&mut self) {
        fn action(ast: &mut Ast) {
            let mut stmts = ast.node_stack.pop().unwrap();
            //let args = ast.node_stack.pop().unwrap();
            // arg reductions shoulve been added to the value when reduced
            let _ = ast.node_stack.pop().unwrap();
            stmts.token = Tok::FuncDecl;
            // some how need to set the name of stmts
            // maybe by having a value of (name, args)
            // stmts.val = Func(name, args);
            ast.node_stack.push(stmts);
            println!("reducing func decl");
        }

        self.install_prod(Tok::Stmt, &vec![
            Tok::FnKW,
            Tok::Var,
            Tok::LeftParen,
            Tok::ArgList,
            Tok::RightParen,
            Tok::Block],
            Some(action));
    }

    pub fn install_stmt_var_decl(&mut self) {
        fn action(ast: &mut Ast) {
            let var = ast.node_stack.pop().unwrap();
            let mut eq = ast.node_stack.pop().unwrap();
            let expr = ast.node_stack.pop().unwrap();

            eq.children.push(var);
            eq.children.push(expr);

            ast.node_stack.push(eq);
        }

        self.install_prod(Tok::Stmt, &vec![
            Tok::LetKW,
            Tok::Var,
            Tok::Eq,
            Tok::Expr,
            Tok::SemiColon],
        Some(action));
    }

    pub fn install_stmt_expr(&mut self) {
        self.install_prod(Tok::Stmt, &vec![
            Tok::Expr,
            Tok::SemiColon],
        None);
    }
}