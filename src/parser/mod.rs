use crate::ast::{Ast, NodeVal};
use crate::lexer::Lexer;
use crate::tokens::{keyword_check, Tok};
use bovidae::{Bovidae, ParseResult};
use lexify::{LexifyToken, LexifyError};

pub mod var_list;
pub mod binop;
pub mod block;
pub mod expr;
pub mod expr_list;
pub mod stmt;
pub mod stmts;
pub mod call;
pub mod decl;

type ProdID = usize;

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

    pub fn build_ast(&mut self, lexer: &mut Lexer, ast: &mut Ast) -> Result<(), ()> {
        loop {
            let lex_tok = lexer.next_token()?;

            match lex_tok {
                LexifyToken::Eof => {
                    self.parse_end(ast);
                    break;
                }
                LexifyToken::Tok(tid, attr) => self.parse(tid, attr, ast),
            }
        }

        Ok(())
    }

    fn shift_node(&mut self, tok: Tok, attr: Option<&str>, ast: &mut Ast) {
        if tok.non_semantic_token() {
            return;
        }

        match tok {
            Tok::String => ast.push_node(
                Tok::String,
                Some(NodeVal::String(attr.unwrap().to_string())),
            ),
            Tok::Int => ast.push_node(
                Tok::Int,
                Some(NodeVal::Int(
                    attr.unwrap().to_string().parse::<i32>().unwrap(),
                )),
            ),
            Tok::Var => {
                let sym_id = ast.get_sym_id(attr.unwrap());

                ast.push_node(Tok::Var, Some(NodeVal::Sym(sym_id)));
            }
            _ => ast.push_node(tok, None),
        }
    }

    fn reduce_node(&mut self, pid: ProdID, ast: &mut Ast) {
        if let Some(action) = self.reduction_actions[pid] {
            action(ast);
        }
    }

    fn process_raw_tid(&self, raw_tid: Tok, attr: Option<&str>) -> Tok {
        match raw_tid {
            Tok::Var => {
                if let Some(keyword) = keyword_check(attr.unwrap()) {
                    return keyword;
                }

                Tok::Var
            }
            _ => raw_tid,
        }
    }

    pub fn parse(&mut self, raw_tid: Tok, attr: Option<&str>, ast: &mut Ast) {
        self.parse_until_shift(Some(self.process_raw_tid(raw_tid, attr)), attr, ast);
    }

    fn parse_until_shift(&mut self, tid: Option<Tok>, attr: Option<&str>, ast: &mut Ast) {
        loop {
            let parse_result = self.parser.parse(tid);

            if parse_result.is_err() {
                panic!("Parsing error")
            } else {
                match parse_result.ok().unwrap() {
                    ParseResult::Accept => {
                        //println!("==== PARSER ACCEPTED :) ====");
                        break;
                    }
                    ParseResult::Reduction(_, pid) => {
                        // TODO remove tid from ParseResult

                        self.reduce_node(pid, ast);
                        continue;
                    }
                    ParseResult::Shift => {
                        self.shift_node(tid.unwrap(), attr, ast);
                        break;
                    }
                }
            }
        }
    }

    fn parse_end(&mut self, ast: &mut Ast) {
        self.parse_until_shift(Some(Tok::End), None, ast);
        self.parse_until_shift(None, None, ast);
    }

    pub fn install_prod(&mut self, head: Tok, body: &Vec<Tok>, action: Option<fn(&mut Ast)>) {
        let tok_id_body = body.iter().collect();

        self.reduction_actions.push(action);

        self.parser.set_prod(head, &tok_id_body)
    }

    fn install_prods(&mut self) {
        // start
        self.install_start(); // START => STMTS

        // stmts
        self.install_stmts_list(); // STMTS => STMT STMTS
        self.install_stmts_empty(); // STMTS => EMPTY

        // block
        self.install_block(); // Block => { Stmts }

        // stmt
        self.install_stmt_decl(); // STMT => DECL
        self.install_stmt_control(); // STMT => CONTROL
        self.install_stmt_expr(); // STMT => EXPR ;

        // DECL
        self.install_decl_var();
        self.install_decl_func();

        // CONTROL

    }
}
