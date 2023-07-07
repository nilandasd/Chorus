pub type TokID = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tok {
    Start,

    Stmts,
    Stmt,

    Block,

    Expr,

    FuncDecl,
    Decl,
    Control,

    End,

    FnKW,
    VarList,

    Var,
    Int,
    String,
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,
    SemiColon,
    Minus,
    Eq,
    Comma,
    Plus
}

pub fn keyword_check(word: &str) -> Option<Tok> {
    match word {
        /*
        "fn" => Some(Tok::FnKW),
        "if" => Some(Tok::IfKW),
        "return" => Some(Tok::ReturnKW),
        */
        _ => None,
    }
}


impl Tok {
    pub fn non_semantic_token(&self) -> bool {
        match self {
            Tok::LeftCurly
            | Tok::RightCurly
            | Tok::LeftParen
            | Tok::RightParen
            | Tok::SemiColon
            | Tok::Comma
            /*
            | Tok::FnKW
            | Tok::LetKW
            | Tok::ReturnKW
            */
            | Tok::End => true,
            _ => false,
        }
    }
}
