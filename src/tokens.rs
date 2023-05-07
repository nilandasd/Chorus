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
    ReturnKW,

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
    VarList,
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
        "return" => Some(Tok::ReturnKW),
        _ => None,
    }
}

pub fn tid_to_tok(tid: TokID) -> Tok {
    match tid {
        0 => Tok::Var,
        1 => Tok::Int,
        2 => Tok::String,
        3 => Tok::FnKW,
        4 => Tok::LetKW,
        5 => Tok::IfKW,
        6 => Tok::ReturnKW,
        7 => Tok::LeftCurly,
        8 => Tok::RightCurly,
        9 => Tok::LeftParen,
        10 => Tok::RightParen,
        11 => Tok::SemiColon,
        12 => Tok::Comma,
        13 => Tok::Eq,
        14 => Tok::Plus,
        15 => Tok::Minus,
        16 => Tok::End,

        // nonterms
        17 => Tok::Start,
        18 => Tok::Stmts,
        19 => Tok::Stmt,
        20 => Tok::FuncCall,
        21 => Tok::FuncDecl,
        22 => Tok::ExprStmt,
        23 => Tok::StmtDecl,
        24 => Tok::ExprList,
        25 => Tok::VarList,
        26 => Tok::Block,
        27 => Tok::Expr,
        28 => Tok::NestedExpr,
        29 => Tok::BinExpr,
        30 => Tok::Value,
        31 => Tok::BinOp,
        _ => panic!("Token does not exist"),
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
            | Tok::FnKW
            | Tok::LetKW
            | Tok::ReturnKW
            | Tok::End => true,
            _ => false,
        }
    }
}
