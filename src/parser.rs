use crate::grammar::NT;
use crate::grammar::Symbol;

struct Item<'a> {
  head: NT,
  body: &'a [Symbol],
  la: Symbol
}

struct State<'a> {
  id: u8,
  items: &'a [Item]
}

impl Item {
  fn postfix(self) -> &[Symbol] {
    
  }

  fn expects(self) -> Symbol {
    let iter = self.body.iter();

    for symbol in iter {
      match symbol {
        Cursor => {
          if let Some(s) = iter.next() {
            return s
          } else {
            return None
          }
        }
        _ => {}
      }
    }

    return None
  }
}
