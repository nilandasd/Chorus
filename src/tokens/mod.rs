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
        10 =>  Tok::SemiColon,
        11 => Tok::Comma,
        12 => Tok::Eq,
        13 => Tok::Plus,
        14 => Tok::End,

        // nonterms // im sexy and im sexy
        15 => Tok::Start,
        16 => Tok::Stmts,
        17 => Tok::Stmt,
        18 => Tok::FuncCall,
        19 => Tok::FuncDecl,
        20 => Tok::ExprStmt,
        21 => Tok::StmtDecl,
        22 => Tok::ExprList,
        23 => Tok::ArgList,
        24 => Tok::Block,
        25 => Tok::Expr,
        26 => Tok::NestedExpr,
        27 => Tok::BinExpr,
        28 => Tok::Value,
        29 => Tok::BinOp,
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

