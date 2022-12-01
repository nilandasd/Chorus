use crate::grammar::NT;
use crate::grammar::Symbol;

#[derive(PartialEq, Eq)]
struct Item<'a> {
  head: NT,
  body: &'a [Symbol],
  la: Symbol
}

#[derive(PartialEq, Eq)]
struct State<'a> {
  id: u8,
  items: &'a [Item<'a>]
}

impl<'a> Item<'a> {
  fn new(head: NT, symbol_vec: Vec<Symbol>, la: Symbol) -> &'a Item<'a> {
    let body: &[Symbol] = &symbol_vec;
    &Self { head, body, la }
  }

  fn expects(self) -> Symbol {
    let mut flag = 0;
    for symbol in self.body.iter() {
      if flag == 1 {
        return symbol.clone();
      }

      match symbol {
        Symbol::Cursor => {
          flag = 1;
        }
        _ => {}
      }
    }

    Symbol::None
  }

  fn postfix(self) -> Vec<Symbol> {
    let mut result = vec![];
    let mut flag = 0;

    for symbol in self.body.iter() {
      if flag == 1 {
        result.push(symbol.clone());
        continue
      }

      match symbol {
        Symbol::Cursor => {
          flag = 1;
        }
        _ => {}
      }

    }

    result
  }

  fn shift_cursor(self) -> Vec<Symbol> {
    let mut result = vec![];
    let mut flag = 0;

    for symbol in self.body.iter() {
      match flag {
        0 => {
          if Symbol::Cursor == symbol.clone() {
            flag = 1;
          } else {
            result.push(symbol.clone());
          }
        }
        1 => {
          result.push(symbol.clone());
          result.push(Symbol::Cursor);
          flag = 2;
        }
        2 => {
          result.push(symbol.clone());
        }
      }
    }
    
    if flag == 1 { panic!("could not shift the cursor!") }

    result
  }
}

impl<'a> State<'a> {
  fn goto(self, symbol: Symbol) {
    todo!();
  }

  // two different kinds of closures
}
