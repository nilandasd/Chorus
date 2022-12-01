use std::fs::File;
use std::cmp::Ordering;
use std::io::Read;
use std::collections::VecDeque;

pub struct Lex {
  tokens: VecDeque<Token>,
  fh: FileHandle
}

pub enum Token {
  Var(String),
  String(String),
  Char(u8),
  Int(i32),
  Float(f64),
  Op(OPID),
  Type(Type),
  Simple(TID),
}

// Token ID
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TID {
  // -- ATTR TOKENS --- 
  Var, Int, Float, Char, String, Op, Type,

  // --- SIMPLE TOKENS ---

  // Memory Ops
  Val, Adr,

  // Delimiters
  RParen, LParen, RBrack, LBrack, RCurly, LCurly, Assign, Semi, Colon, Comma,

  // Reserved Words (not including type words)
  Main, For, While, Func, Return, If, Else, New, Free,

  //End Token
  End,
}

pub const DELIMITERS: [(&str, TID); 10] = [
  ("{", TID::LCurly),
  ("}", TID::RCurly),
  ("(", TID::LParen),
  (")", TID::RParen),
  ("[", TID::LBrack),
  ("]", TID::RBrack),
  ("=", TID::Assign),
  (";", TID::Semi  ),
  (":", TID::Colon ),
  (",", TID::Comma ),
];

// Operator ID
#[derive(Copy, Clone)]
pub enum OPID {
  Neq, Eq, Gte, Lte, Lt, Gt, Add, Sub, Mul, Div, Mod, Or, And, Neg,
}

// not including '*' and '&'
pub const OPERATORS: [(&str, OPID); 14] = [
  ("!=", OPID::Neq),
  ("==", OPID::Eq ),
  (">=", OPID::Gte),
  ("<=", OPID::Lte),
  ("<",  OPID::Lt ),
  (">",  OPID::Gt ),
  ("+",  OPID::Add),
  ("-",  OPID::Sub),
  ("*",  OPID::Mul),
  ("/",  OPID::Div),
  ("%",  OPID::Mod),
  ("||", OPID::Or ),
  ("&&", OPID::And),
  ("!",  OPID::Neg),
];

#[derive(Copy, Clone)]
pub enum Type {
  Char,
  Int,
  Float,
}

pub const TYPE_WORDS: [(&str, Type); 3] = [
  ("float",  Type::Float ),
  ("int",    Type::Int   ),
  ("char",   Type::Char  ),
];

pub const RESERVED_WORDS: [(&str, TID); 9] = [
  ("for",    TID::For       ),
  ("while",  TID::While     ),
  ("fn",     TID::Func      ),
  ("return", TID::Return    ),
  ("if",     TID::If        ),
  ("else",   TID::Else      ),
  ("new",    TID::New       ),
  ("free",   TID::Free      ),
  ("main",   TID::Main      ),
];

impl Lex {
  pub fn new(file: File) -> Lex {
    Lex {
      tokens: VecDeque::new(),
      fh: FileHandle::new(file)
    }
  }

  pub fn read_tokens(&mut self) {
    loop {
      let char = self.fh.get_char();

      match char {
        Some(c) => {
          if c.is_ascii_whitespace() {
            continue;

          } else if c == b'#' {
            self.skip_line();

          } else if c.is_ascii_alphabetic() {
            self.read_word(c); 

          } else if c.is_ascii_digit() {
            self.read_num(c); 

          } else if c == b'\'' {
            self.read_char();

          } else if c == b'"' {
            self.read_string();

          } else if op(c) {
            self.read_op(c);

          } else if delimiter(c) {
            self.read_delimiter(c);

          } else {
            todo!("panic on unaccepted token");
          }
        }

        None => { self.tokens.push_back(Token::Simple(TID::End)); break; }
      }
    }
  }

  pub fn get_token(&mut self) -> Option<Token> {
    self.tokens.pop_front()
  }

  fn skip_line(&mut self) {
    loop {
      match self.fh.get_char() {
        Some(c) => { if new_line(c) { break; }}
        None => { break; }
      }
    }
  }

  fn read_word(&mut self, c: u8) {
    let mut w = String::new();
    w.push(c as char);

    loop {
      match self.fh.peek() {
        Some(c) => {
          if letter(c) {
            w.push(c as char);
            self.fh.get_char();
          } else {
            self.push_word(&w);
            break;
          }
        }
        None => { 
          self.push_word(&w);
          break;
        }
      }
    }
  }

  fn push_word(&mut self, word: &String) {
    if let Some(tid) = reserved_tid(&word) {
      self.tokens.push_back(Token::Simple(tid));
    } else if let Some(type_id) = reserved_type(&word) {
      self.tokens.push_back(Token::Type(type_id));
    } else {
      self.tokens.push_back(Token::Var(word.to_string()));
    }
  }

  fn read_num(&mut self, c: u8) {
    let mut num_str = String::new();
    num_str.push(c as char);

    loop {
      match self.fh.peek() {
        Some(c) => {
          if number(c) {
            num_str.push(c as char);
            self.fh.get_char();
          } else if c == b'.' {
            num_str.push(c as char);
            self.fh.get_char();
            self.read_float(&mut num_str);
            break;
          } else {
            self.tokens.push_back(Token::Int(num_str.parse().unwrap()));
            break;
          }
        }
        None => { 
          self.tokens.push_back(Token::Int(num_str.parse().unwrap()));
          break;
        }
      }
    }
  }

  fn read_float(&mut self, float_str: &mut String) {
    loop {
      match self.fh.peek() {
        Some(c) => {
          if number(c) {
            float_str.push(c as char);
            self.fh.get_char();
          } else {
            self.tokens.push_back(Token::Float(float_str.parse().unwrap()));
            break;
          }
        }
        None => { 
          self.tokens.push_back(Token::Float(float_str.parse().unwrap()));
          break;
        }
      }
    }
  }

  fn read_string(&mut self) {
    let mut str = String::new();

    loop {
      match self.fh.peek() {
        Some(c) => {
          if double_quote(c) {
            self.tokens.push_back(Token::String(str.to_string()));
            self.fh.get_char();
            break;
          } else {
            str.push(c as char);
            self.fh.get_char();
          }
        }
        None => { 
          todo!("put a panic here! string was not closed!")
        }
      }
    }
  }

  fn read_char(&mut self) {
    let char = self.fh.get_char();
    if let Some(c) = self.fh.peek() {
      if c != b'\'' {
        todo!("panic: char not closed");
      }
    } else {
      todo!("panic: char not closed");
    }
    self.fh.get_char();

    self.tokens.push_back(Token::Char(char.unwrap()));
  }

  fn read_op(&mut self, c: u8) {
    let mut s = String::new();
    s.push(c as char);

    let cc = self.fh.peek();
    if peek_required(c) && cc.is_some() {
      s.push(cc.unwrap() as char);

      if let Some(opid) = operator_opid(&s) {
        self.fh.get_char();
        self.tokens.push_back(Token::Op(opid));
        return
      }

      s.pop();
    }


    if s == "$" {
      self.tokens.push_back(Token::Simple(TID::Val));
    } else if s == "@" {
      self.tokens.push_back(Token::Simple(TID::Adr));
    } else if let Some(opid) = operator_opid(&s) {
      self.tokens.push_back(Token::Op(opid));
    } else {
      todo!("should've matched with an op here")
    }
  }

  fn read_delimiter(&mut self, c: u8) {
    let mut s = String::new();
    s.push(c as char);

    self.tokens.push_back(Token::Simple(delimiter_tid(&s).unwrap()));
  }
}

// ============================================================================
// ==============  PRIVATE FUNCTIONS  =========================================
// ============================================================================

fn delimiter_tid(word: &String) -> Option<TID> {
  for res in DELIMITERS.iter() {
    let res_str = res.0;
    let res_id = res.1;

    if res_str == word {
      return Some(res_id)
    } 
  }

  None
}

fn reserved_tid(word: &String) -> Option<TID> {
  for res in RESERVED_WORDS.iter() {
    let res_str = res.0;
    let res_id = res.1;

    if res_str == word {
      return Some(res_id)
    } 
  }

  None
}

fn reserved_type(word: &String) -> Option<Type> {
  for res in TYPE_WORDS.iter() {
    let res_str = res.0;
    let res_id = res.1;

    if res_str == word {
      return Some(res_id)
    } 
  }

  None
}

fn operator_opid(word: &String) -> Option<OPID> {
  for res in OPERATORS.iter() {
    let res_str = res.0;
    let res_id = res.1;

    if res_str == word {
      return Some(res_id)
    } 
  }

  None
}

fn delimiter(c: u8) -> bool {
  (c == b'(') || (c == b')') || (c == b'[') ||
  (c == b']') || (c == b'{') || (c == b'}') ||
  (c == b',') || (c == b'=') || (c == b';')
}

fn op(c: u8) -> bool {
  (c == b'&') || (c == b'|') || (c == b'!') ||
  (c == b'+') || (c == b'-') || (c == b'*') ||
  (c == b'/') || (c == b'%') || (c == b'=') ||
  (c == b'<') || (c == b'>') || (c == b'$') ||
  (c == b'@')
}

fn peek_required(c: u8) -> bool {
  (c == b'&') || (c == b'|') || (c == b'!') ||
  (c == b'=') || (c == b'<') || (c == b'>')
}

fn new_line(c: u8) -> bool {
  c == b'\n'
}

// ============================================================================
// ==============  FILE HANDLER ===============================================
// ============================================================================

const BUFFER_LEN: usize = 8000;

struct FileHandle {
  file: File,
  read_count: usize,
  cursor: usize,
  buf: [u8; BUFFER_LEN]
}

impl FileHandle {
  fn new(f: File) -> FileHandle {
    FileHandle {
      file: f,
      read_count: BUFFER_LEN,
      cursor: BUFFER_LEN,
      buf: [0; BUFFER_LEN]
    }
  }

  fn get_char(&mut self) -> Option<u8> {
    if BUFFER_LEN <= self.cursor {
      if self.read_count != BUFFER_LEN {
        return None;
      } else {
        let res = self.file.read(&mut self.buf);
        if res.is_err() {
          panic!("failed to read file")
        }
        self.read_count = res.unwrap();
        if self.read_count == 0 {
          return None;
        }
        self.cursor = 1;
        let c = self.buf[0];
        Some(c)
      }
    } else {
      if self.cursor < self.read_count {
        let c = self.buf[self.cursor];
        self.cursor += 1;
        Some(c)
      } else {
        None
      }
    }
  }

  fn peek(&mut self) -> Option<u8> {
    match self.cursor.cmp(&BUFFER_LEN) {
      Ordering::Less => { 
        Some(self.buf[self.cursor])
      },
      _ => { None },
    }
  }
}
