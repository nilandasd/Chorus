use crate::generator::Generator;
use crate::tokens::Tok;
use std::collections::HashMap;

type SymID = usize;
type NodeID = usize;
const PRINT_SYM_ID: usize = 0;
const PRINT_FN_ID: usize = 0;

#[derive(Debug)]
pub struct Ast {
    pub node_stack: Vec<Node>, // at end of parsing node_stack[0] is the ast root
    pub symbol_table: HashMap<String, SymID>,
    node_counter: usize,
}

#[derive(Debug)]
pub enum NodeVal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Sym(SymID),
}

#[derive(Debug)]
pub struct Node {
    pub token: Tok,
    pub id: NodeID,
    pub children: Vec<Node>,
    pub val: Option<NodeVal>,
    //start_label: Option<NodeID>,
    //end_jump: Option<NodeID>,
    //end_label: Option<NodeID>,
}

impl Node {
    pub fn has_const_val(&self) -> bool {
        match self.val.as_ref() {
            Some(NodeVal::Sym(_)) | None => false,
            _ => true,
        }
    }

    pub fn traverse(&mut self, generator: &mut Generator) {
        self.preprocess_node(generator);

        for child in self.children.iter_mut().rev() {
            child.traverse(generator);
        }

        self.process_node(generator);
    }

    fn preprocess_node(&mut self, generator: &mut Generator) {
        match self.token {
            Tok::FuncDecl => {
                generator.gen_func_enter(self);
            }

            // IF ELSE STMT --------
            // label1 = id of child 1
            // label2 = id of child 2
            // gen expr
            // gen JNE to label 1
            // process child 1
            // gen jump label2
            // gen label1
            // process child 2
            // gen label2

            // WHILE STMT --------
            // gen label1
            // JNE to label2
            // process block
            // jump to label 1
            // label 2

            _ => {}
        }
    }

    fn process_node(&mut self, generator: &mut Generator) {
        match self.token {
            Tok::Plus | Tok::Minus => generator.gen_expr(self),
            Tok::Eq => generator.gen_decl(self),
            Tok::FuncDecl => generator.gen_func_leave(self),
            Tok::FuncCall => generator.gen_func_call(self),
            Tok::ReturnKW => generator.gen_return(self),
            Tok::VarList => generator.gen_var_list(self),
            _ => {}
        }
    }
}

impl Ast {
    pub fn init() -> Self {
        Self {
            node_stack: vec![],
            symbol_table: Ast::init_symbol_table(),
            node_counter: 1,
        }
    }

    fn init_symbol_table() -> HashMap<String, SymID> {
        let mut symbol_table = HashMap::<String, SymID>::new();

        symbol_table.insert("print".to_string(), PRINT_SYM_ID);

        symbol_table
    }

    pub fn synthesize_expr(&mut self, op: Tok, left_val: NodeVal, right_val: NodeVal) {
        match op {
            Tok::Plus => match (left_val, right_val) {
                (NodeVal::Int(l), NodeVal::Int(r)) => {
                    self.push_node(Tok::Int, Some(NodeVal::Int(l + r)));
                }
                _ => {}
            },
            Tok::Minus => match (left_val, right_val) {
                (NodeVal::Int(l), NodeVal::Int(r)) => {
                    self.push_node(Tok::Int, Some(NodeVal::Int(l - r)));
                }
                _ => {}
            },
            _ => {}
        }
    }
    
    pub fn new_node(&mut self, token: Tok, val: Option<NodeVal>) -> Node {
        self.node_counter += 1;

        Node {
            id: self.node_counter,
            token,
            val,
            children: vec![],
        }
    }

    pub fn push_node(&mut self, token: Tok, val: Option<NodeVal>) {
        let new_node = self.new_node(token, val);
        self.node_stack.push(new_node);
    }

    pub fn get_sym_id(&mut self, sym: &str) -> SymID {
        let num_symbols = self.symbol_table.len();

        match self.symbol_table.get(sym) {
            Some(sym_id) => *sym_id,
            None => {
                self.symbol_table.insert(sym.to_string(), num_symbols);
                num_symbols
            }
        }
    }

    pub fn traverse(&mut self, generator: &mut Generator) {
        self.node_stack[0].traverse(generator);
        generator.complete_generation();
    }

    pub fn display(&self) {
        if self.node_stack.is_empty() {
            println!("AST IS EMPTY :(");
            return;
        }

        let root = &self.node_stack[0];

        self.print_node(root, 0);
    }

    fn print_node(&self, node: &Node, depth: usize) {
        for _ in 0..depth {
            print!("\t");
        }

        print!("{:?}", node.token);

        match &node.val {
            Some(NodeVal::String(val)) => print!(" :: {}", val),
            Some(NodeVal::Int(val)) => print!(" :: {}", val),
            Some(NodeVal::Sym(sym_id)) => print!(" :: {}", sym_id),
            _ => {}
        }

        if !node.children.is_empty() {
            print!("\t{{");
        } else {
            println!("");
            return;
        }

        println!("");
        for child in node.children.iter().rev() {
            self.print_node(child, depth + 1);
        }
        for _ in 0..depth {
            print!("\t");
        }
        print!("}}");
        println!("");
    }
}