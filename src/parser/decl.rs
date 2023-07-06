
use crate::ast::Ast;
use crate::parser::Parser;
use crate::tokens::Tok;

impl Parser {
    pub fn install_decl_func(&mut self) {
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
            Tok::Decl,
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

    pub fn install_decl_var(&mut self) {
        fn action(ast: &mut Ast) {
            let expr = ast.node_stack.pop().unwrap();
            let mut eq = ast.node_stack.pop().unwrap();
            let var = ast.node_stack.pop().unwrap();

            eq.val = var.val;
            eq.children.push(expr);

            ast.node_stack.push(eq);
        }

        self.install_prod(
            Tok::Decl, 
            &vec![Tok::Var, Tok::Eq, Tok::Expr], 
            Some(action));
    }
}
