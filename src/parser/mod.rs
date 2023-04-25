use bovidae::{Bovidae, ParseResult};  
use lexify::{LexifyToken, LexifyError};
use crate::tokens::{Tok, TokID, tid_to_tok, keyword_check};
use crate::ast::{Ast, Node, Value};
use crate::lexer::Lexer;
use crate::generator::Generator;

pub mod stmts;
pub mod stmt;
pub mod block;
pub mod expr_list;
pub mod arg_list;
pub mod expr;
pub mod binop;

type ProdID = usize;
pub struct Parser {
    parser: Bovidae,
    lexer: Lexer,
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
            lexer: Lexer::init(),
            reduction_actions: Vec::<Option<fn(&mut Ast)>>::new(),
        }
    }

    pub fn build_ast(&mut self, lexer: &mut Lexer, ast: &mut Ast) {
        loop {
            let lex_tok = lexer.next_token().ok().unwrap();

            //println!("{:?}", lex_tok);

            match lex_tok {
                LexifyToken::Eof => self.parse_end(ast),
                LexifyToken::Tok(tid, attr) => self.parse(tid, attr, ast),
            }

            if let LexifyToken::Eof = lex_tok {
                break;
            }
        }
    }

    fn shift_node(&mut self, tok: Tok, attr: Option<&str>, ast: &mut Ast) {
        if tok.non_semantic_token() { return }

        match tok {
            Tok::String => {
                ast.node_stack.push(Node {
                    token: Tok::String,
                    children: vec![],
                    attr: Some(Value::String(attr.unwrap().to_string()))
                })
            }
            Tok::Int => {
                ast.node_stack.push(Node {
                    token: Tok::Int,
                    children: vec![],
                    attr: Some(Value::Int(attr.unwrap().to_string().parse::<i32>().unwrap()))
                })
            }
            Tok::Var => {
                let sym_id = ast.get_sym_id(attr.unwrap());

                ast.node_stack.push(Node {
                    token: Tok::Var,
                    children: vec![],
                    attr: Some(Value::Sym(sym_id))
                })
            }
            _ => ast.node_stack.push(Node::new(tok)),
        }
    }

    fn reduce_node(&mut self, pid: ProdID, ast: &mut Ast) {
        if let Some(action) = self.reduction_actions[pid] {
            action(ast);
        }
    }

    fn process_raw_tid(&self, raw_tid: TokID, attr: Option<&str>) -> TokID {
        match tid_to_tok(raw_tid) {
            Tok::Var =>  {
                if let Some(keyword) = keyword_check(attr.unwrap()) {
                    keyword as TokID
                } else {
                    Tok::Var as TokID
                }
            }
            _ => raw_tid,
        }
    }

    pub fn parse(&mut self, raw_tid: TokID, attr: Option<&str>, ast: &mut Ast) {
        self.parse_until_shift(Some(self.process_raw_tid(raw_tid, attr)), attr, ast);
    }

    fn parse_until_shift(&mut self, tid: Option<TokID>, attr: Option<&str>, ast: &mut Ast) {
        loop {
            let parse_result = self.parser.parse(tid);

            if parse_result.is_err() {
                panic!("cringe, no error recovery")
            } else {
                match parse_result.ok().unwrap() {
                    ParseResult::Accept => { println!("ACCEPTED :) ~!!!!~~!!~!~!"); break; }
                    ParseResult::Reduction(_, pid) => {
                        // TODO remove tid from ParseResult

                        self.reduce_node(pid, ast);
                        continue;
                    }
                    ParseResult::Shift => {
                        self.shift_node(tid_to_tok(tid.unwrap()), attr, ast);
                        break;
                    }
                }
            }
        }
    }

    fn parse_end(&mut self, ast: &mut Ast) {
        self.parse_until_shift(Some(Tok::End as TokID), None, ast);
        self.parse_until_shift(None, None, ast);
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
        self.install_expr_int();
        self.install_expr_var();

        // binop
        self.install_binop_plus();
    }
}