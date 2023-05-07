use crate::parser::Parser;
use crate::tokens::Tok;
use crate::ast::Ast;

impl Parser {
    pub fn install_arg_list_comma(&mut self) {
        fn action(ast: &mut Ast) {
            let mut var_list = ast.node_stack.pop().unwrap();
            let var = ast.node_stack.pop().unwrap();

            var_list.children.push(var);
            ast.node_stack.push(var_list);
        }

        self.install_prod(
            Tok::VarList, 
            &vec![Tok::Var, Tok::Comma, Tok::VarList], 
            Some(action)
        );
    }

    pub fn install_arg_list_last(&mut self) {
        fn action(ast: &mut Ast) {
            let var = ast.node_stack.pop().unwrap();
            let mut var_list = ast.new_node(Tok::VarList, None);

            var_list.children.push(var);
            ast.node_stack.push(var_list);
        }

        self.install_prod(Tok::VarList, &vec![Tok::Var], Some(action));
    }

    pub fn install_arg_list_empty(&mut self) {
        fn action(ast: &mut Ast) {
            ast.push_node(Tok::VarList, None);
        }

        self.install_prod(Tok::VarList, &vec![], Some(action));
    }
}
