use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_expr_list_comma(&mut self) {
        fn action(ast: &mut Ast) {
            let mut expr_list = ast.node_stack.pop().unwrap();
            let expr = ast.node_stack.pop().unwrap();

            expr_list.children.push(expr);
            ast.node_stack.push(expr_list);
        }

        self.install_prod(
            Tok::ExprList,
            &vec![Tok::Expr, Tok::Comma, Tok::ExprList],
            Some(action),
        );
    }

    pub fn install_expr_list_last(&mut self) {
        fn action(ast: &mut Ast) {
            let expr = ast.node_stack.pop().unwrap();
            let mut expr_list = ast.new_node(Tok::ExprList, None);

            expr_list.children.push(expr);
            ast.node_stack.push(expr_list);
        }

        self.install_prod(Tok::ExprList, &vec![Tok::Expr], Some(action));
    }

    pub fn install_expr_list_empty(&mut self) {
        fn action(ast: &mut Ast) {
            ast.push_node(Tok::ExprList, None);
        }

        self.install_prod(Tok::ExprList, &vec![], Some(action));
    }
}
