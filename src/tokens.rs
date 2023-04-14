pub type TokID = usize;

#[derive(Debug)]
pub enum Tok {
    Var,
    Int,
    String,

    // key words
    FnKW,
    LetKW,

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

pub fn keyword_check(word: &str) -> Option<TokID> {
    match word {
        "fn" => Some(Tok::FnKW as TokID),
        "let" => Some(Tok::LetKW as TokID),
        _ => None,
    }
}

pub fn tid_to_token(tid: TokID) -> Tok {
    match tid {
        0 =>  Tok::Var,
        1 =>  Tok::Int,
        2 =>  Tok::String,
        3 =>  Tok::FnKW,
        4 =>  Tok::LetKW,
        5 =>  Tok::LeftCurly,
        6 =>  Tok::RightCurly,
        7 =>  Tok::LeftParen,
        8 =>  Tok::RightParen,
        9 =>  Tok::SemiColon,
        10 => Tok::Comma,
        11 => Tok::Eq,
        12 => Tok::Plus,
        13 => Tok::End,
        14 => Tok::Start,
        15 => Tok::Stmts,
        16 => Tok::Stmt,
        17 => Tok::FuncCall,
        18 => Tok::FuncDecl,
        19 => Tok::ExprStmt,
        20 => Tok::StmtDecl,
        21 => Tok::ExprList,
        22 => Tok::ArgList,
        23 => Tok::Block,
        24 => Tok::Expr,
        25 => Tok::NestedExpr,
        26 => Tok::BinExpr,
        27 => Tok::Value,
        28 => Tok::BinOp,
        _ => panic!("Token does not exist")
    }
}
