use bovidae::{Bovidae, ParseResult};  
use lexify::{LexifyToken, LexifyError};
use crate::tokens::{Tok, TokID};
use crate::ast::{Ast, Node};
use crate::lexer::Lexer;

pub mod stmts;
pub mod stmt;
pub mod block;
pub mod expr_list;
pub mod arg_list;
pub mod expr;
pub mod binop;

pub struct Parser {
    parser: Bovidae,
    reduction_actions: Vec<Option<fn(&mut Ast)>>,
}

impl Parser {
    pub fn init() -> Self {
        let mut parser = Parser::new();
        parser.install_prods();
        parser.parser.generate_parser();
        parser
    }

    pub fn new() -> Self {
        Self {
            parser: Bovidae::new(),
            reduction_actions: Vec::<Option<fn(&mut Ast)>>::new(),
        }
    }

    pub fn build_ast(&mut self, ast: &mut Ast, lexer: &mut Lexer) {
        loop {
            let token = lexer.next_token().ok().unwrap();

            println!("{:?}", token);

            match token {
                LexifyToken::Eof => break,
                _ => continue,
            }
        }
    }

    pub fn parse(&mut self, tid: Option<TokID>) -> Result<ParseResult, ()> {
        self.parser.parse(tid)
    }

    pub fn install_prod(&mut self, head: Tok, body: &Vec<Tok>, action: Option<fn(&mut Ast)>) {
        let tok_id_body = body.iter()
            .map(|tok| *tok as TokID)
            .collect();

        self.reduction_actions.push(action);

        self.parser.set_prod(head as TokID, &tok_id_body)
    }

    fn install_prods(&mut self) {
        // start
        self.install_start();

        // stmts
        self.install_stmts_list();
        self.install_stmts_last();

        // block
        self.install_block();

        // stmt
        self.install_stmt_func_decl();
        self.install_stmt_var_decl();
        self.install_stmt_expr();

        // Expr List
        self.install_expr_list_comma();
        self.install_expr_list_last();
        self.install_expr_list_empty();

        // Arg List
        self.install_arg_list_comma();
        self.install_arg_list_last();
        self.install_arg_list_empty();

        // Expr
        self.install_expr_func_call();
        self.install_expr_nested();
        self.install_expr_binop();
        self.install_expr_string();
        self.install_expr_var();

        // binop
        self.install_binop_plus();
    }
}