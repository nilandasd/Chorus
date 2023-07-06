use crate::tokens::{Tok, TokID};
use lexify::{Lexify, LexifyError, LexifyToken};
use std::fs::File;
use std::io::BufReader;

pub struct Lexer {
    lexer: lexify::Lexify<BufReader<File>, Tok>,
}

impl Lexer {
    pub fn init() -> Self {
        let mut lexer = Lexer::new();

        lexer.install_ignores();
        lexer.install_terms();

        lexer
    }

    pub fn next_token(&mut self) -> Result<LexifyToken<Tok>, ()> {
        match self.lexer.next_token() {
            Ok(t) => Ok(t),
            Err(lex_err) => {
                // add lex error to errors
                Err(())
            }
        }
    }

    fn new() -> Self {
        Self {
            lexer: Lexify::new(),
        }
    }

    pub fn open_file(&mut self, path: &str) -> Result<(), ()> {
        let f = File::open(path);

        if f.is_err() {
            println!("Unable to open file: {}", path);
            return Err(());
        }

        let reader = BufReader::new(f.ok().unwrap());

        self.lexer.set_buf_reader(reader);

        Ok(())
    }

    fn install_ignores(&mut self) {
        self.lexer.set_ignore("\\w+");
        self.lexer.set_ignore(r#"/\*.*\*/"#);
        self.lexer.set_ignore("//(.|^)+");
    }

    fn install_terms(&mut self) {
        // Terminals with Values
        self.lexer.set_rule(r#"\l(\l|\d|_)*"#, Tok::Var,    true);
        self.lexer.set_rule(r#"\d+"#,          Tok::Int,    true);
        self.lexer.set_rule(r#""(.|\\.)*""#,   Tok::String, true);

        // one char terminals
        self.lexer.set_rule(r#"{"#,  Tok::LeftCurly,  false);
        self.lexer.set_rule(r#"}"#,  Tok::RightCurly, false);
        self.lexer.set_rule(r#"\("#, Tok::LeftParen,  false);
        self.lexer.set_rule(r#"\)"#, Tok::RightParen, false);
        self.lexer.set_rule(r#";"#,  Tok::SemiColon,  false);
        self.lexer.set_rule(r#"="#,  Tok::Eq,         false);
        self.lexer.set_rule(r#"\+"#, Tok::Plus,       false);
        self.lexer.set_rule(r#"-"#,  Tok::Minus,      false);
        self.lexer.set_rule(r#","#,  Tok::Comma,      false);
    }
}
