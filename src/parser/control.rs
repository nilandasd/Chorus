use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_control_if(&mut self) {
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
            Tok::Control,
            &vec![
                Tok::IfKW,
                Tok::Expr,
                Tok::Block,
            ],
            Some(action),
        );
    }
}
