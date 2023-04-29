use crate::ast::{Node, NodeVal};
use crate::tokens::Tok;

type SymID = usize;
type TempID = usize;
type FnID = usize;
type ObjID = usize;
type ListID = usize;
type LabelID = usize;
type NodeID = usize;

#[derive(Debug)]
enum IRval {
    Nil,
    Int(i32),
    String(String),
    Sym(SymID),
    Temp(TempID),
    Obj(ObjID),
    Fn(FnID),
    List(ListID),
}

#[derive(Debug)]
enum IRop {
    Plus,
    Minus,
}

#[derive(Debug)]
enum IRline {
    Op(IRval, IRop, IRval, IRval),
    Load(IRval, IRval),
    Label(LabelID),
    Jump(LabelID),
    JmpIf(LabelID, IRval),
    Call(IRval),
    Return(IRval),
    End,
}

struct FuncDef {
    code: Vec<IRline>,
}
pub struct Generator {
    code: Vec<IRline>,
    funcs: Vec<FuncDef>,
    func_stack: Vec<FnID>, // the top of the stack is the current function being defined
}

impl Generator {
    pub fn init() -> Self {
        Self {
            code: Vec::<IRline>::new(),
            funcs: vec![],
            func_stack: vec![]
        }
    }

    fn push_line(&mut self, line: IRline) {
        if self.func_stack.is_empty() {
            self.code.push(line);
        } else {
            self.funcs[*(self.func_stack.last().unwrap())].code.push(line);
        }
    }

    pub fn gen_expr(&mut self, expr_node: &mut Node) {
        let left_val = Generator::node_val_to_ir_val(&expr_node.children[1]);
        let right_val = Generator::node_val_to_ir_val(&expr_node.children[0]);
        let ir_op = Generator::tok_op_to_ir_op(expr_node.token);
        let op_line = IRline::Op(IRval::Temp(expr_node.id), ir_op, left_val, right_val);

        self.push_line(op_line);
    }

    pub fn gen_decl(&mut self, decl_node: &mut Node) {
        if let Some(NodeVal::Sym(sym_id)) = decl_node.val {
            let child_node = &decl_node.children[0];
            let ir_val = Generator::node_val_to_ir_val(&child_node);
            let load_line = IRline::Load(IRval::Sym(sym_id), ir_val);

            self.push_line(load_line);
        }
    }

    fn push_label(&mut self, label: NodeID) {
        self.push_line(IRline::Label(label));
    }

    pub fn gen_end(&mut self) {
        self.push_line(IRline::End);
    }

    pub fn gen_func_enter(&mut self, func_decl: &mut Node) {
        // set the symbol to be equal to the new func ID
        if let Some(NodeVal::Sym(sym_id)) = func_decl.val {
            self.push_line(IRline::Load(IRval::Sym(sym_id), IRval::Fn(func_decl.id)));
        }

        self.func_stack.push(self.funcs.len());
        self.funcs.push(FuncDef { 
            code: vec![]
        });

        self.push_label(func_decl.id);

        // label of the nodeID
        // gen set all the args
    }

    pub fn gen_func_call(&mut self, func_call: &mut Node) {
        if let Some(NodeVal::Sym(sym_id)) = func_call.val {
            self.push_line(IRline::Call(IRval::Sym(sym_id)));
        }
        //let new_line = IRline::Call(func_call.val)
    }

    pub fn gen_func_leave(&mut self, func_decl: &mut Node) {
        if let Some(IRline::Return(ir_val)) = self.funcs[*(self.func_stack.last().unwrap())].code.last() {
            // don't need to add a return statement
        } else {
            self.push_line(IRline::Return(IRval::Nil));
        }

        self.func_stack.pop();
    }

    fn node_val_to_ir_val(node: &Node) -> IRval {
        match node.val.as_ref() {
            None => IRval::Temp(node.id),
            Some(val) => match val {
                NodeVal::Int(i) => IRval::Int(*i),
                NodeVal::Sym(s) => IRval::Sym(*s),
                NodeVal::String(s) => IRval::String(s.clone()),
                _ => todo!("cant convert to this val"),
            },
        }
    }

    fn tok_op_to_ir_op(tok_op: Tok) -> IRop {
        match tok_op {
            Tok::Plus => IRop::Plus,
            Tok::Minus => IRop::Minus,
            _ => todo!("have yet to implement this op"),
        }
    }

    pub fn display(&self) {
        println!("----- IR CODE -----");

        for line in self.code.iter() {
            println!("{:?}", line);
        }

        println!("----- END -----");
        println!("");
        println!("----- FUNC CODE -----");

        for func in self.funcs.iter() {
            for line in func.code.iter() {
                println!("{:?}", line);
            }
        }
    }
}
