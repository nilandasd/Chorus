
use lexify::{Lexify, LexifyToken, LexifyError};
use std::io::BufReader;
use std::fs::File;
use crate::tokens::{Tok, TokID};

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

    pub fn next_token(&mut self) -> Result<LexifyToken, LexifyError> {
        self.lexer.next_token()
    }

    fn new() -> Self {
        Self {
            lexer: Lexify::new(),
        }
    }

    pub fn open_file(&mut self, path: &str) -> std::io::Result<()> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        self.lexer.set_buf_reader(reader);

        Ok(())
    }

    fn install_ignores(&mut self) {
        self.lexer.set_ignore("\\w+");
    }

    fn install_terms(&mut self) {
        // Terminals with Values
        self.lexer.set_rule(r#"\l(\l|\d|_)*"#, Tok::Var as TokID, true);
        self.lexer.set_rule(r#"\d+"#, Tok::Int as TokID, true);
        self.lexer.set_rule(r#""(.|\\.)*""#, Tok::String as TokID, true);

        // one char terminals
        self.lexer.set_rule(r#"{"#, Tok::LeftCurly as TokID, false);
        self.lexer.set_rule(r#"}"#, Tok::RightCurly as TokID, false);
        self.lexer.set_rule(r#"\("#, Tok::LeftParen as TokID, false);
        self.lexer.set_rule(r#"\)"#, Tok::RightParen as TokID, false);
        self.lexer.set_rule(r#";"#, Tok::SemiColon as TokID, false);
        self.lexer.set_rule(r#"="#, Tok::Eq as TokID, false);
        self.lexer.set_rule(r#"\+"#, Tok::Plus as TokID, false);
    }
}