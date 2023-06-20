use crate::tokens::{Tok, TokID};
use lexify::{Lexify, LexifyError, LexifyToken};
use std::fs::File;
use std::io::BufReader;

pub struct Lexer {
    lexer: lexify::Lexify<BufReader<File>>,
}

impl Lexer {
    pub fn init() -> Self {
        let mut lexer = Lexer::new();

        lexer.install_ignores();
        lexer.install_terms();

        lexer
    }

    pub fn next_token(&mut self) -> Result<LexifyToken, ()> {
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
        self.lexer
            .set_rule(r#"\l(\l|\d|_)*"#, Tok::Var as TokID, true);
        self.lexer.set_rule(r#"\d+"#, Tok::Int as TokID, true);
        self.lexer
            .set_rule(r#""(.|\\.)*""#, Tok::String as TokID, true);

        // one char terminals
        self.lexer.set_rule(r#"{"#, Tok::LeftCurly as TokID, false);
        self.lexer.set_rule(r#"}"#, Tok::RightCurly as TokID, false);
        self.lexer.set_rule(r#"\("#, Tok::LeftParen as TokID, false);
        self.lexer
            .set_rule(r#"\)"#, Tok::RightParen as TokID, false);
        self.lexer.set_rule(r#";"#, Tok::SemiColon as TokID, false);
        self.lexer.set_rule(r#"="#, Tok::Eq as TokID, false);
        self.lexer.set_rule(r#"\+"#, Tok::Plus as TokID, false);
        self.lexer.set_rule(r#"-"#, Tok::Minus as TokID, false);
        self.lexer.set_rule(r#","#, Tok::Comma as TokID, false);
    }
}
