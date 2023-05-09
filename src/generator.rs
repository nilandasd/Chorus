use crate::ast::{Node, NodeVal};
use crate::tokens::Tok;
use std::collections::HashMap;

type SymID = usize;
type TempID = usize;
type FnID = usize;
type ObjID = usize;
type StrID = usize;
type ListID = usize;
type LabelID = usize;
type NodeID = usize;
type ArgID = usize;

#[derive(Debug, Clone)]
pub enum IRval {
    Nil,
    Int(i32),
    String(String),
    StrID(StrID),
    Sym(SymID),
    Temp(TempID),
    Arg,
    Obj(ObjID),
    ObjAccessor(ObjID, SymID),
    Fn(FnID),
    List(ListID),
}

#[derive(Debug, Clone, Copy)]
pub enum IRop {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum IRline {
    Op(IRval, IRop, IRval, IRval),
    Load(IRval, IRval),
    Label(LabelID),
    Jump(LabelID),
    JumpIf(LabelID, IRval),
    Call(IRval, IRval),
    Return(IRval),
    Print(IRval),
    End,
}

struct FuncDef {
    code: Vec<IRline>,
}
pub struct Generator {
    pub code: Vec<IRline>,
    funcs: Vec<FuncDef>,
    func_stack: Vec<FnID>, // the top of the stack is the current function being defined
    strings: Vec<String>,
}

impl Generator {
    pub fn init() -> Self {
        let mut generator = Self {
            code: vec![],
            funcs: vec![],
            func_stack: vec![],
            strings: vec![]
        };

        generator.init_code();

        generator
    }

    pub fn complete_generation(&mut self) {
        self.push_line(IRline::End);
        self.concat_functions();
        self.remove_labels();
    }

    fn concat_functions(&mut self) {
        for func in self.funcs.iter() {
            for line in func.code.iter() {
                self.code.push(line.clone());
            }
        }
    }

    fn remove_labels(&mut self) {
        let mut fixed_code = Vec::<IRline>::new();
        let mut label_positions = HashMap::<usize, usize>::new();

        // first pass remove labels and save their positions
        for line in self.code.iter() {
            if let IRline::Label(label_id) = line {
                label_positions.insert(*label_id, fixed_code.len());
            } else {
                fixed_code.push(line.clone());
            }
        }

        //println!("{:?}", label_positions);
        // second pass remove labels
        for line in fixed_code.iter_mut() {
            match line {
                IRline::Load(from_id, _) => {
                    if let IRval::Fn(id) = from_id {
                        *from_id = IRval::Fn(*(label_positions.get(id).unwrap()));
                    }
                }
                _ => {}
            }
        }

        self.code = fixed_code;
    }

    fn init_code(&mut self) {
        self.create_print_func();
    }

    fn create_print_func(&mut self) {
        self.push_line(IRline::Load(IRval::Fn(0), IRval::Sym(0)));

        self.func_stack.push(self.funcs.len());
        self.funcs.push(FuncDef { 
            code: vec![]
        });

        self.push_label(0);
        self.push_line(IRline::Print(IRval::Arg));
        self.push_line(IRline::Return(IRval::Nil));
        self.func_stack.pop();
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

    pub fn gen_return(&mut self, return_node: &mut Node) {
        let return_line = IRline::Return(Generator::node_val_to_ir_val(&return_node.children[0]));

        self.push_line(return_line);
    }

    pub fn gen_decl(&mut self, decl_node: &mut Node) {
        if let Some(NodeVal::Sym(sym_id)) = decl_node.val {
            let child_node = &decl_node.children[0];
            let ir_val = Generator::node_val_to_ir_val(&child_node);
            let load_line = IRline::Load(ir_val, IRval::Sym(sym_id));

            self.push_line(load_line);
        }
    }

    fn push_label(&mut self, label: NodeID) {
        self.push_line(IRline::Label(label));
    }

    pub fn gen_func_enter(&mut self, func_decl: &mut Node) {
        if let Some(NodeVal::Sym(sym_id)) = func_decl.val {
            self.push_line(IRline::Load(IRval::Fn(func_decl.id), IRval::Sym(sym_id)));
        }

        self.func_stack.push(self.funcs.len());
        self.funcs.push(FuncDef { 
            code: vec![]
        });

        self.push_label(func_decl.id);
    }

    pub fn gen_func_call(&mut self, func_call: &mut Node) {
        for arg in func_call.children.iter() {
            self.push_line(IRline::Load(Generator::node_val_to_ir_val(arg), IRval::Arg));
        }

        if let Some(NodeVal::Sym(sym_id)) = func_call.val {
            self.push_line(IRline::Call(IRval::Temp(func_call.id), IRval::Sym(sym_id)));
        }
    }

    pub fn gen_var_list(&mut self, var_list: &mut Node) {
        for arg in var_list.children.iter().rev() {
            if let Some(NodeVal::Sym(sym_id)) = arg.val {
                self.push_line(IRline::Load(IRval::Arg, IRval::Sym(sym_id)));
            }
        }
    }

    pub fn gen_func_leave(&mut self) {
        if let Some(IRline::Return(_)) = self.funcs[*(self.func_stack.last().unwrap())].code.last() {
            // don't need to add a return statement
        } else {
            self.push_line(IRline::Return(IRval::Nil));
        }

        self.func_stack.pop();
    }

    fn node_val_to_ir_val(node: &Node) -> IRval {
        match node.token {
            Tok::FuncCall => return IRval::Temp(node.id),
            _ => {}
        }

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
        let mut line_num = 0;

        for line in self.code.iter() {
            println!("{}: {:?}", line_num, line);
            line_num += 1;
        }
    }
}
