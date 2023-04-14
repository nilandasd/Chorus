use crate::tokens::{Tok, TokID, tid_to_token};

type TypeID = usize;
pub struct Ast {
    pub node_stack: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i8),
    //Args(Vec<TokID>)
}

#[derive(Debug)]
pub struct Node {
    pub token: Tok,
    pub children: Vec<Node>,
    pub val: Option<Value>,
    pub type_id: Option<TypeID>,
    // kind: 
    // scope
}

impl Ast {
    pub fn init() -> Self {
        Self {
            node_stack: vec![],
        }
    }

    pub fn display(&self) {
        if self.node_stack.is_empty() { 
            println!("AST IS EMPTY");
            return
        }

        let root = &self.node_stack[0];

        self.print_node(root, 0);
    }

    fn print_node(&self, node: &Node, depth: usize) {
        for _ in 0..depth {
            print!("\t");
        }
        if node.children.is_empty() {
            print!("{:?} :: {:?}", node.token, node.val);
            println!("");
            return; 
        }
        print!("{:?}\t{{", node.token);
        println!("");
        for child in node.children.iter() {
            self.print_node(child, depth + 1);
        }
        for _ in 0..depth {
            print!("\t");
        }
        print!("}}");
        println!("");
    }
}

fn shift_leaf(ast: &mut Ast, token_id: TokID, _: Option<&str>) {
    if token_id == Tok::LeftCurly as TokID
        || token_id == Tok::RightCurly as TokID 
        || token_id == Tok::LeftParen as TokID
        || token_id == Tok::RightParen as TokID
        || token_id == Tok::SemiColon as TokID
        || token_id == Tok::FnKW as TokID
        || token_id == Tok::LetKW as TokID
    {
            return
    }

    ast.node_stack.push(Node {
        token: tid_to_token(token_id),
        val: None,
        children: vec![],
        type_id: None,
    });
}