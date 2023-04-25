use crate::tokens::Tok;
use crate::generator::Generator;
use std::collections::HashMap;

type SymID = usize;
type ObjID = usize;
type FnID = usize;

#[derive(Debug)]
pub struct Ast {
    pub node_stack: Vec<Node>, // at end of parsing node_stack[0] is the ast root
    pub symbol_table: HashMap::<String, SymID>,
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    List(Vec<Box<Value>>),
    Obj(ObjID),
    Fn(FnID), // usize for arg count
    Sym(SymID)
}

#[derive(Debug)]
pub struct Node {
    pub token: Tok,
    pub children: Vec<Node>,
    pub attr: Option<Value>,
}

impl Node {
    pub fn new(token: Tok) -> Self {
        Self {
            token,
            children: vec![],
            attr: None,
        }
    }

    pub fn traverse(&mut self, generator: &mut Generator) {
        self.enter_node(generator);

        for child in self.children.iter_mut().rev() {
            child.traverse(generator);
        }

        self.leave_node(generator);
    }

    fn enter_node(&mut self, generator: &mut Generator) {
        println!("entering: {:?}", self.token);
    }

    fn leave_node(&mut self, generator: &mut Generator) {
        match self.token {

            _ => {}
        }
        println!("leaving: {:?}", self.token);
    }
}

impl Ast {
    pub fn init() -> Self {
        Self {
            node_stack: vec![],
            symbol_table: HashMap::<String, SymID>::new(),
        }
    }

    pub fn get_sym_id(&mut self, sym: &str) -> SymID {
        let num_symbols = self.symbol_table.len();

        match self.symbol_table.get(sym) {
            Some(sym_id) => *sym_id,
            None => {
                self.symbol_table.insert(sym.to_string(), num_symbols);
                num_symbols
            },
       }
    }

    pub fn traverse(&mut self) {
        let mut generator = Generator {};
        self.node_stack[0].traverse(&mut generator);
    }

    pub fn display(&self) {
        if self.node_stack.is_empty() { 
            println!("AST IS EMPTY :(");
            return
        }

        let root = &self.node_stack[0];

        self.print_node(root, 0);
    }

    fn print_node(&self, node: &Node, depth: usize) {
        for _ in 0..depth {
            print!("\t");
        }

        print!("{:?}", node.token);

        match &node.attr {
            Some(Value::String(val)) => print!(" :: {}", val),
            Some(Value::Int(val)) => print!(" :: {}", val),
            Some(Value::Sym(sym_id)) => print!(" :: {}", sym_id),
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