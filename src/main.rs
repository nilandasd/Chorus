mod lex;
mod grammar;
use std::fs::File;

fn main() -> std::io::Result<()> {
  let filename = "test";
  let file = File::open(filename)?;
  let mut lex = lex::Lex::new(file);

  lex.read_tokens();

  let mut t = lex.get_token();
  while t.is_some() {
    t.as_ref().map(|tok| {
      match tok {
        lex::Token::Var(_) => {
          println!("var");
        }
        lex::Token::Float(_) => {
          println!("float");
        }
        lex::Token::Int(_) => {
          println!("int");
        }
        lex::Token::String(_) => {
          println!("string");
        }
        lex::Token::Char(_) => {
          println!("char");
        }
        lex::Token::Op(_) => {
          println!("op");
        }
        lex::Token::Type(_) => {
          println!("type");
        }
        lex::Token::Simple(_) => {
          println!("simple");
        }
      }
    });
    t = lex.get_token();
  }

  Ok(())
}
