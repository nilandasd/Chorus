use crate::ast::{Node, NodeVal};
use crate::tokens::Tok;

type SymID = usize;
type TempID = usize;
type FnID = usize;
type ObjID = usize;
type ListID = usize;
type LabelID = usize;

#[derive(Debug)]
enum IRval {
    Int(i32),
    Sym(SymID),
    Temp(TempID),
    Obj(ObjID),
    Fn(FnID),
    List(ListID),
}

#[derive(Debug)]
enum IRop {
    Plus,
    Minus
}

#[derive(Debug)]
enum IRline {
    Op(IRval, IRop, IRval, IRval),
    Load(IRval, IRval),
    Label(LabelID),
    Jump(LabelID),
    JmpIf(LabelID, IRval),
    Call(IRval, Vec<IRval>),
    Return(IRval)
}

pub struct Block {
    
}
pub struct Generator {
    code: Vec<IRline>,
    //blocks: Vec<Block>,
}

impl Generator {
    pub fn init() -> Self {
        Self {
            code: Vec::<IRline>::new(),
        }
    }

    pub fn gen_expr(&mut self, expr_node: &mut Node) {
        let left_val = Generator::node_val_to_ir_val(&expr_node.children[1]);
        let right_val = Generator::node_val_to_ir_val(&expr_node.children[0]);
        let ir_op = Generator::tok_op_to_ir_op(expr_node.token);
        let op_line = IRline::Op(IRval::Temp(expr_node.id), ir_op, left_val, right_val);

        self.code.push(op_line);
    }

    pub fn gen_decl(&mut self, decl_node: &mut Node) {
        if let Some(NodeVal::Sym(sym_id)) = decl_node.val {
            let child_node = &decl_node.children[0];
            let ir_val = Generator::node_val_to_ir_val(&child_node);
            let load_line = IRline::Load(IRval::Sym(sym_id), ir_val);

            self.code.push(load_line);
        }
    }

    fn node_val_to_ir_val(node: &Node) -> IRval {
        match node.val.as_ref() {
            None => IRval::Temp(node.id),
            Some(val) => {
                match val {
                    NodeVal::Int(i) => IRval::Int(*i),
                    NodeVal::Sym(s) => IRval::Sym(*s),
                    _ => todo!("cant convert to this val"),
                }
            }
        }
    }

    fn tok_op_to_ir_op(tok_op: Tok) -> IRop {
        match tok_op {
            Tok::Plus =>  IRop::Plus,
            Tok::Minus => IRop::Minus,
            _ => todo!("have yet to implement this op")
        }
    }

    pub fn display(&self) {
        println!("----- IR CODE -----");

        for line in self.code.iter() {
            println!("{:?}", line);
        }
    }
}