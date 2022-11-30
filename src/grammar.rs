use crate::lex::TID;

// Non-Terminal
enum NT {
  Stmts, Stmt, Decl, ArrIndex, Type, ClosedStmt, Block, Expr, LV, RV, Params,
  ReturnType, Args, ParamsList, ArgsList
}

enum Symbol {
  Term(TID),
  NonTerm(NT),
  Cursor,
  None
}

enum PID {
  // Stmts
  Stmts, LastStmts,

  // Block
  Block,

  // Decl
  HeapDecl, StackDecl, FuncDecl,

  // ArrIndex
  ArrIndex, EmptyArrIndex,

  // Type
  RegType, PtrType,

  // ReturnType
  ReturnType, EmptyReturnType,

  // Stmt
  ClosedStmt, BlockStmt, IfStmt, IfElseStmt, WhileStmt,

  // ClosedStmt
  DeclStmt, DeclInitStmt, ExprStmt, AssignStmt, ReturnStmt,

  // LV
  VarLV, ArrLV, ValLV,

  // RV
  IntRV, FloatRV, CharRV, StringRV, AdrRV, CallRV, LVRV,

  // Expr
  NestExpr, OpExpr, RVExpr,

  // Params
  Params, EmptyParams,
  ListParams, LastParams,
  
  // Args
  Args, EmptyArgs,
  ListArgs, LastArgs
}

struct Prod<'a> {
  pid: PID,
  head: NT,
  body: &'a [Symbol],
}

const fn n(n: NT) -> Symbol {
  Symbol::NonTerm(n)
}

const fn t(n: TID) -> Symbol {
  Symbol::Term(n)
}

const fn prod(pid: PID, head: NT, body: &[Symbol]) -> Prod {
  Prod {
    pid,
    head,
    body,
  }
}

/*
 * STMTS =>
 *       |  Stmts Stmt
 *       |  Stmt
 *
 * BLOCK =>
 *      |  { Stmts }
 *
 * DECL =>
 *      |  Type Var ArrIndex
 *      |  New Type Var ArrIndex
 *      |  Func Var Params ReturnType Block
 *
 * ArrIndex =>
 *      | [ Int ] ArrIndex
 *      | EMPTY
 *
 * TYPE =>
 *      | Type
 *      | Type @
 *
 * RETURN_TYPE =>
 *      | : Type ArrIndex
 *      | EMPTY
 * 
 * STMT => 
 *      | ClosedStmt ;
 *      | Block
 *      | If Expr Block
 *      | If Expr Block Else Block
 *      | While Expr Block
 *
 * ClosedStmt =>
 *      | Decl
 *      | Decl = Expr
 *      | Expr
 *      | LV = Expr
 *      | Return Args
 *
 * LV =>
 *      | Var
 *      | Var ArrIndex
 *      | $ LV 
 *
 * RV =>
 *      | Attr
 *      | @ Var
 *      | Var Args
 *      | LV
 *
 * EXPR =>
 *      | ( Expr )
 *      | Expr Op Expr
 *      | RV
 *
 * PARAMS =>
 *      | ( Params )
 *      | ( )
 *
 * PARAMS_LIST=>
 *      | Type ArrIndex , ParamsList
 *      | ParamsList
 *
 * ARGS =>
 *      | ( Args )
 *      | ( )
 *
 * ARGS_LIST =>
 *      | Expr, ArgsList
 *      | Expr
 */

pub const GRAMMAR: [Prod; 43] = [
  // Stmts
  Prod {
    pid:  PID::Stmts,
    head: NT::Stmts,
    body: &[n(NT::Stmts), n(NT::Stmt)]
  },
  Prod {
    pid:  PID::LastStmts,
    head: NT::Stmts,
    body: &[n(NT::Stmt)]
  },

  // Block
  Prod {
    pid:  PID::Block,
    head: NT::Block,
    body: &[t(TID::LCurly), n(NT::Stmt), t(TID::RCurly)]
  },

  // Decl
  Prod {
    pid:  PID::StackDecl,
    head: NT::Decl,
    body: &[n(NT::Type), t(TID::Var), ]
  },
  Prod {
    pid:  PID::HeapDecl,
    head: NT::Decl,
    body: &[t(TID::New), n(NT::Type), t(TID::Var)]
  },
  Prod {
    pid:  PID::FuncDecl,
    head: NT::Decl,
    body: &[t(TID::Func), t(TID::Var), n(NT::Params),
            n(NT::ReturnType), n(NT::Block)]
  },

  // ArrIndex
  Prod {
    pid:  PID::ArrIndex,
    head: NT::ArrIndex,
    body: &[t(TID::New), n(NT::Type), t(TID::Var)]
  },
  Prod {
    pid:  PID::EmptyArrIndex,
    head: NT::ArrIndex,
    body: &[Symbol::None]
  },

  // Type
  Prod {
    pid:  PID::RegType,
    head: NT::Type,
    body: &[t(TID::Type)]
  },
  Prod {
    pid:  PID::PtrType,
    head: NT::Type,
    body: &[t(TID::Type), t(TID::Adr)]
  },

  // Return Type
  Prod {
    pid:  PID::ReturnType,
    head: NT::ReturnType,
    body: &[t(TID::Colon), n(NT::Type), n(NT::ArrIndex)]
  },
  Prod {
    pid:  PID::EmptyReturnType,
    head: NT::ReturnType,
    body: &[Symbol::None]
  },

  // Stmt
  Prod {
    pid:  PID::ClosedStmt,
    head: NT::Stmt,
    body: &[n(NT::ClosedStmt), t(TID::Semi)]
  },
  Prod {
    pid:  PID::BlockStmt,
    head: NT::Stmt,
    body: &[n(NT::Block)]
  },
  Prod {
    pid:  PID::IfStmt,
    head: NT::Stmt,
    body: &[t(TID::If), n(NT::Expr), n(NT::Block)]
  },
  Prod {
    pid:  PID::IfElseStmt,
    head: NT::Stmt,
    body: &[t(TID::If), n(NT::Expr), n(NT::Block),
            t(TID::Else), n(NT::Block)]
  },
  Prod {
    pid:  PID::WhileStmt,
    head: NT::Stmt,
    body: &[t(TID::While), n(NT::Expr), n(NT::Block)]
  },

  // ClosedStmt
  Prod {
    pid:  PID::DeclStmt,
    head: NT::ClosedStmt,
    body: &[n(NT::Decl)]
  },
  Prod {
    pid:  PID::DeclInitStmt,
    head: NT::ClosedStmt,
    body: &[n(NT::Decl), t(TID::Assign), n(NT::Expr)]
  },
  Prod {
    pid:  PID::ExprStmt,
    head: NT::ClosedStmt,
    body: &[n(NT::Expr)]
  },
  Prod {
    pid:  PID::AssignStmt,
    head: NT::ClosedStmt,
    body: &[n(NT::LV), t(TID::Assign), n(NT::Expr)]
  },
  Prod {
    pid:  PID::ReturnStmt,
    head: NT::ClosedStmt,
    body: &[t(TID::Return), n(NT::Args)]
  },

  // LV
  Prod {
    pid:  PID::VarLV,
    head: NT::LV,
    body: &[t(TID::Var)]
  },
  Prod {
    pid:  PID::ArrLV,
    head: NT::LV,
    body: &[t(TID::Var), n(NT::ArrIndex)]
  },
  Prod {
    pid:  PID::ValLV,
    head: NT::LV,
    body: &[t(TID::Val), n(NT::LV)]
  },

  // RV
  Prod {
    pid:  PID::IntRV,
    head: NT::RV,
    body: &[t(TID::Int)]
  },
  Prod {
    pid:  PID::FloatRV,
    head: NT::RV,
    body: &[t(TID::Float)]
  },
  Prod {
    pid:  PID::CharRV,
    head: NT::RV,
    body: &[t(TID::Char)]
  },
  Prod {
    pid:  PID::StringRV,
    head: NT::RV,
    body: &[t(TID::String)]
  },
  Prod {
    pid:  PID::AdrRV,
    head: NT::RV,
    body: &[t(TID::Val), t(TID::Var)]
  },
  Prod {
    pid:  PID::CallRV,
    head: NT::RV,
    body: &[t(TID::Var), n(NT::Args)]
  },
  Prod {
    pid:  PID::LVRV,
    head: NT::RV,
    body: &[n(NT::LV)]
  },

  // EXPR
  Prod {
    pid:  PID::NestExpr,
    head: NT::Expr,
    body: &[t(TID::LParen), n(NT::LV), t(TID::RParen)]
  },
  Prod {
    pid:  PID::OpExpr,
    head: NT::Expr,
    body: &[n(NT::Expr), t(TID::Op), n(NT::Expr)]
  },
  Prod {
    pid:  PID::RVExpr,
    head: NT::Expr,
    body: &[n(NT::RV)]
  },

  // PARAMS
  Prod {
    pid:  PID::Params,
    head: NT::Params,
    body: &[t(TID::LParen), n(NT::ParamsList), t(TID::RParen)]
  },
  Prod {
    pid:  PID::EmptyParams,
    head: NT::Params,
    body: &[t(TID::LParen), t(TID::RParen)]
  },
  Prod {
    pid:  PID::ListParams,
    head: NT::ParamsList,
    body: &[n(NT::Type), n(NT::ArrIndex), t(TID::Comma),
            n(NT::ParamsList)]
  },
  Prod {
    pid:  PID::LastParams,
    head: NT::ParamsList,
    body: &[n(NT::Type), n(NT::ArrIndex)]
  },
  
  // ARGS
  Prod {
    pid:  PID::Args,
    head: NT::Args,
    body: &[t(TID::LParen), n(NT::ArgsList), t(TID::RParen)]
  },
  Prod {
    pid:  PID::EmptyArgs,
    head: NT::Args,
    body: &[t(TID::LParen), t(TID::RParen)]
  },
  Prod {
    pid:  PID::ListArgs,
    head: NT::ArgsList,
    body: &[n(NT::Expr), t(TID::Comma), n(NT::ArgsList)]
  },
  Prod {
    pid:  PID::LastArgs,
    head: NT::ArgsList,
    body: &[n(NT::Expr)]
  },
];

