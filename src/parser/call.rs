use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_call(&mut self) {
        fn action(ast: &mut Ast) {
            let expr_list = ast.node_stack.pop().unwrap();
            let mut var = ast.node_stack.pop().unwrap();

            var.token = Tok::FuncCall;
            for child in expr_list.children {
                var.children.push(child);
            }
            ast.node_stack.push(var);
        }

        self.install_prod(
            Tok::FuncCall,
            &vec![Tok::Var, Tok::LeftParen, Tok::ExprList, Tok::RightParen],
            Some(action),
        );
    }
}