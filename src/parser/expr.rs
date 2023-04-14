use crate::parser::Parser;
use crate::ast::{Ast, Node};
use crate::tokens::Tok;

impl Parser {
    pub fn install_expr_func_call(&mut self) {
        fn action(ast: &mut Ast) {
            let expr_list = ast.node_stack.pop().unwrap();
            let mut var = ast.node_stack.pop().unwrap();

            var.token = Tok::FuncCall;
            var.children.push(expr_list);

            ast.node_stack.push(var);

            println!("reducing expr func call");
        }

        self.install_prod(Tok::Expr, &vec![
            Tok::Var,
            Tok::LeftParen,
            Tok::ExprList,
            Tok::RightParen],
            Some(action));
    }

    pub fn install_expr_nested(&mut self) {
        self.install_prod(Tok::Expr, &vec![
            Tok::LeftParen,
            Tok::Expr,
            Tok::RightParen],
            None);
    }

    pub fn install_expr_binop(&mut self) {
        fn action(ast: &mut Ast) {
            let right_expr = ast.node_stack.pop().unwrap();
            let mut op = ast.node_stack.pop().unwrap();
            let left_expr = ast.node_stack.pop().unwrap();

            op.children.push(left_expr);
            op.children.push(right_expr);
            ast.node_stack.push(op);
        }

        self.install_prod(Tok::Expr, &vec![
            Tok::Expr,
            Tok::BinOp,
            Tok::Expr],
            Some(action));
    }

    pub fn install_expr_string(&mut self) {
        self.install_prod(Tok::Expr, &vec![Tok::String], None);
    }

    pub fn install_expr_var(&mut self) {
        self.install_prod(Tok::Expr, &vec![Tok::Var], None);
    }
}