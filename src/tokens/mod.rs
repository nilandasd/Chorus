pub type TokID = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tok {
    Var,
    Int,
    String,

    // key words
    FnKW,
    LetKW,
    IfKW,

    // one char terms
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,
    SemiColon,
    Comma,
    Eq,
    Plus,
    Minus,
    End,

    Start,
    Stmts,
    Stmt,
    FuncCall,
    FuncDecl,
    ExprStmt,
    StmtDecl,
    ExprList,
    ArgList,
    Block,
    Expr,
    NestedExpr,
    BinExpr,
    Value,
    BinOp,

}

pub fn keyword_check(word: &str) -> Option<Tok> {
    match word {
        "fn" => Some(Tok::FnKW),
        "if" => Some(Tok::IfKW),
        _ => None,
    }
}

pub fn tid_to_tok(tid: TokID) -> Tok {
    match tid {
        0 =>  Tok::Var,
        1 =>  Tok::Int,
        2 =>  Tok::String,
        3 =>  Tok::FnKW,
        4 =>  Tok::LetKW,
        5 =>  Tok::IfKW,
        6 =>  Tok::LeftCurly,
        7 =>  Tok::RightCurly,
        8 =>  Tok::LeftParen,
        9 =>  Tok::RightParen,
        10 => Tok::SemiColon,
        11 => Tok::Comma,
        12 => Tok::Eq,
        13 => Tok::Plus,
        14 => Tok::Minus,
        15 => Tok::End,

        // nonterms // im sexy and im sexy
        16 => Tok::Start,
        17 => Tok::Stmts,
        18 => Tok::Stmt,
        19 => Tok::FuncCall,
        20 => Tok::FuncDecl,
        21 => Tok::ExprStmt,
        22 => Tok::StmtDecl,
        23 => Tok::ExprList,
        24 => Tok::ArgList,
        25 => Tok::Block,
        26 => Tok::Expr,
        27 => Tok::NestedExpr,
        28 => Tok::BinExpr,
        29 => Tok::Value,
        30 => Tok::BinOp,
        _ => panic!("Token does not exist")
    }
}

impl Tok {
    pub fn non_semantic_token(&self) -> bool {
        match self {
            Tok::LeftCurly |
            Tok::RightCurly |
            Tok::LeftParen |
            Tok::RightParen |
            Tok::SemiColon |
            Tok::Comma |
            Tok::FnKW |
            Tok::LetKW |
            Tok::End
                => true,
            _ => false,
        }
    }
}

