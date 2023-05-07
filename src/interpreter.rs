use crate::generator::{Generator, IRval, IRline, IRop};

type SymID = usize;
type StrID = usize;
type TempID = usize;

struct StackFrame {
    return_loc: usize,
    temps: Vec<(TempID, IRval)>,
    syms: Vec<(SymID, IRval)>,
}
pub struct Interpreter {
    args: Vec<IRval>,
    position: usize,
    stack: Vec<StackFrame>,
}

impl Interpreter {
    pub fn init() -> Self {
        Self {
            position: 0,
            stack: Interpreter::init_stack(),
            args: vec![],
        }
    }

    fn init_stack() -> Vec<StackFrame> {
        let mut stack = Vec::<StackFrame>::new();

        let global_frame = StackFrame {
            temps: vec![],
            syms: Vec::<(SymID, IRval)>::new(),
            return_loc: 0,
        };

        stack.push(global_frame);

        stack
    }

    pub fn run(&mut self, generator: &mut Generator) {
        loop {
            let code = &generator.code[self.position];

            match code {
                IRline::Op(result, op, left, right) => self.op(*result, *op, *left, *right),
                IRline::Load(val, to) => self.load(*val, *to),
                IRline::Call(val, result) => {}
                IRline::Return(val) => {}
                IRline::Print(val) => self.print(*val),
                IRline::Label(_) => self.next(),
                IRline::Jump(_) => {}
                IRline::JumpIf(_, _) => {}
                IRline::End => break,
            }
            break;
        }
    }

    fn print(&self, val: IRval) {
        match val {
            IRval::Int(i) => println!("{}", i),
            IRval::String(str_id) => println!("{}", self.get_str(str_id)),
            IRval::Arg => self.print(*self.args.last().unwrap()),
            IRval::Nil => println!("nil"),
            IRval::Sym(sym_id) => self.print(self.get_sym_val(sym_id)),
            IRval::Temp(temp_id) => self.print(self.get_sym_val(temp_id)),
            IRval::Fn(fn_id) => todo!("fn"),
            IRval::Obj(obj_id) => todo!("obj"),
            IRval::List(list_id) => todo!("list"),
            IRval::ObjAccessor(_, _) => todo!("obj accessor"),
        }
    }

    fn get_str(&self, str_id: StrID) -> &str {
        todo!()
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn load(&mut self, from: IRval, to: IRval) {
        let copy_val = self.get_copy_val(from);

        match to {
            IRval::Arg => self.args.push(copy_val),
            IRval::Sym(sym_id) => {
                let top_stack = self.stack.len();
                for sym_val in self.stack[top_stack].syms.iter_mut() {
                    if sym_val.0 == sym_id {
                        sym_val.1 = copy_val;
                        return;
                    }
                }
                
                self.stack[top_stack].syms.push((sym_id, copy_val));
            },
            _ => panic!("LOADING INTO NON SYM NON ARG LOCATION"),
        }

        self.next();
    }

    fn op(&mut self, result: IRval, op: IRop, left: IRval, right: IRval) {

    }

    // this function takes an IRval and finds the under lying value to be copied
    // for a load. Primitives like Ints, Chars, Bools, Floats are deep copied.
    // objects, arrays, and strings are shallow copied
    fn get_copy_val(&mut self, val: IRval) -> IRval {
        match val {
            IRval::Arg => self.pop_arg(),
            IRval::Sym(sym_id) => self.get_sym_val(sym_id),
            IRval::Temp(temp_id) => self.get_temp_val(temp_id),
            _ => val,
        }
    }

    fn pop_arg(&mut self) -> IRval {
        self.args.pop().unwrap()
    }

    fn get_sym_val(&self, sym_id: SymID) -> IRval {
        for sframe in self.stack.iter().rev() {
            for sym_val in sframe.syms.iter() {
                if sym_val.0 == sym_id {
                    return sym_val.1;
                }
            }
        }

        panic!("Interpreter: Sym val does not exist!");
    }

    fn get_temp_val(&self, sym_id: TempID) -> IRval {
        for temp_val in self.stack.last().unwrap().temps.iter() {
            if temp_val.0 == sym_id {
                return temp_val.1;
            }
        }

        panic!("Interpreter: Temp val does not exist!");
    }
}