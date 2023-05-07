use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_expr_call(&mut self) {
        self.install_prod(
            Tok::Expr,
            &vec![Tok::FuncCall],
            None,
        );
    }

    pub fn install_expr_nested(&mut self) {
        self.install_prod(
            Tok::Expr,
            &vec![Tok::LeftParen, Tok::Expr, Tok::RightParen],
            None,
        );
    }

    pub fn install_expr_binop(&mut self) {
        fn action(ast: &mut Ast) {
            let right_expr = ast.node_stack.pop().unwrap();
            let mut op = ast.node_stack.pop().unwrap();
            let left_expr = ast.node_stack.pop().unwrap();

            if left_expr.has_const_val() && right_expr.has_const_val() {
                ast.synthesize_expr(op.token, left_expr.val.unwrap(), right_expr.val.unwrap());
            } else {
                op.children.push(left_expr);
                op.children.push(right_expr);
                ast.node_stack.push(op);
            }
        }

        self.install_prod(
            Tok::Expr,
            &vec![Tok::Expr, Tok::BinOp, Tok::Expr],
            Some(action),
        );
    }

    pub fn install_expr_string(&mut self) {
        self.install_prod(Tok::Expr, &vec![Tok::String], None);
    }

    pub fn install_expr_int(&mut self) {
        self.install_prod(Tok::Expr, &vec![Tok::Int], None);
    }

    pub fn install_expr_var(&mut self) {
        self.install_prod(Tok::Expr, &vec![Tok::Var], None);
    }
}
