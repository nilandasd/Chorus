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

struct Parser<'a> {
  states: &'a [State<'a>],
  // propagation table
  // stack
  // action table
  // move table
  // AST
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

fn slr_move(state: State, move_symbol: Symbol) -> State {
  let new_state_items = vec![];
  let mut closure_items: Vec<Item> = Vec::new();
  slr_closure(state, &mut closure_items);

  for item in closure_items {
    if item.expects() == move_symbol {
      let new_item = Item::new(item.head, item.shift_cursor(), item.la);
      new_state_items.push(new_item);
    }
  }

  //let new_state = State {
  //  id: newStateId(),
  //  items: &new_state_items
  //}

  // create a new state
  // break if new state exists
  // add state
  State{id:0, items: Vec::new()}
}

fn slr_closure(state: State, result: &mut Vec<Item>) {
  for old_item in state.items.iter() {
    let symbol_vec = old_item.shift_cursor();
    let new_item = Item::new(old_item.head, symbol_vec, old_item.la);
    result.push(*new_item);
  }
}

impl<'a> Parser<'a> {
  pub fn new() {

  }

  fn init_state() {

  }

  fn slr_move<'func>(self, state: State) -> State {
    let mut closure_items: Vec<Item> = Vec::new();
    self.slr_closure(state, &mut closure_items);

    state
  }

  fn slr_closure(self, state: State, result: &mut Vec<Item>) {
    for old_item in state.items.iter() {
      let symbol_vec = old_item.shift_cursor();
      let new_item = Item::new(old_item.head, symbol_vec, old_item.la);
      result.push(*new_item);
    }
  }

  fn lalr_move() {

  }

  fn lalr_closure() {

  }
}
