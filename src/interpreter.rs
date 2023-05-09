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
    strings: Vec<String>,
}

impl Interpreter {
    pub fn init() -> Self {
        Self {
            position: 0,
            stack: Interpreter::init_stack(),
            args: vec![],
            strings: vec![],
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
            println!("RUNNING : {:?}", code);

            match code {
                IRline::Op(result, op, left, right) => self.op(result.clone(), op.clone(), left.clone(), right.clone()),
                IRline::Load(val, to) => self.load(val.clone(), to.clone()),
                IRline::Call(_, val) => self.call(val.clone()),
                IRline::Return(val) => self.return_call(val.clone()),
                IRline::Print(val) => self.print(val.clone()),
                IRline::Label(_) => {},
                IRline::Jump(_) => {}
                IRline::JumpIf(_, _) => {}
                IRline::End => break,
            }

            self.next();
        }
    }

    fn return_call(&mut self, val: IRval) {
        self.position = self.stack.last().unwrap().return_loc;
        self.stack.pop();
    }

    fn call(&mut self, caller: IRval) {
        let inner_val = self.get_copy_val(caller);

        if let IRval::Fn(fn_id) = inner_val {
            let new_stack = StackFrame {
                temps: vec![],
                syms: Vec::<(SymID, IRval)>::new(),
                return_loc: self.position,
            };

            self.stack.push(new_stack);

            self.position = fn_id - 1;
        } else {
            println!("INTERPRETER ERROR: non function value was called!");
        }
    }

    fn print(&self, val: IRval) {
        match val {
            IRval::Int(i) => println!("{}", i),
            IRval::String(s) => println!("{}", s),
            IRval::StrID(str_id) => println!("{}", self.get_str(str_id)),
            IRval::Sym(sym_id) => self.print(self.get_sym_val(sym_id)),
            IRval::Temp(temp_id) => self.print(self.get_sym_val(temp_id)),
            IRval::Fn(fn_id) => todo!("fn"),
            IRval::Obj(obj_id) => todo!("obj"),
            IRval::List(list_id) => todo!("list"),
            IRval::ObjAccessor(_, _) => todo!("obj accessor"),
            IRval::Arg => self.print(self.args.last().unwrap().clone()),
            IRval::Nil => println!("nil"),
        }
    }

    fn get_str(&self, str_id: StrID) -> &str {
        self.strings[str_id].as_str()
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn load(&mut self, from: IRval, to: IRval) {
        let copy_val = self.get_copy_val(from);

        match to {
            IRval::Arg => self.args.push(copy_val),
            IRval::Sym(sym_id) => {
                let top_stack = self.stack.len() - 1;
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
            IRval::String(s) => self.new_string(s),
            _ => val,
        }
    }

    fn new_string(&mut self, s: String) -> IRval {
        let str_id = self.strings.len();

        self.strings.push(s);

        IRval::StrID(str_id)
    }

    fn pop_arg(&mut self) -> IRval {
        self.args.pop().unwrap()
    }

    fn get_sym_val(&self, sym_id: SymID) -> IRval {
        for sframe in self.stack.iter().rev() {
            for sym_val in sframe.syms.iter() {
                if sym_val.0 == sym_id {
                    return sym_val.1.clone();
                }
            }
        }

        panic!("Interpreter: Sym val does not exist!");
    }

    fn get_temp_val(&self, sym_id: TempID) -> IRval {
        for temp_val in self.stack.last().unwrap().temps.iter() {
            if temp_val.0 == sym_id {
                return temp_val.1.clone();
            }
        }

        panic!("Interpreter: Temp val does not exist!");
    }
}