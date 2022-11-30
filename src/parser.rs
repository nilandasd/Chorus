use crate::grammar::NT;

struct Item<'a> {
  head: NT,
  body: &'a [Symbol],
  la: Symbol
}

struct State<'a> {
  id: u8,
  body: &'a [Item]
}

impl Item {
  fn compare(self, i: &Item) -> bool {
    10
  }

  fn postfix(self) -> Item {
    10
  }

  fn expects(self) -> Symbol {

  }
}

struct SLR {

}

struct LALR {

}


