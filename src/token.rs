use crate::{Action, Builtin, Item};
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"%.*")]
pub enum Token {
    #[token(r"=")]
    PopAndPrint,
    #[token(r"stack")]
    Stack,
    #[regex(r"[-+]?[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Bool(bool),
    #[token(r"counttomark")]
    CountToMark,
    #[token(r"cleartomark")]
    ClearToMark,
    #[token(r"clear")]
    Clear,
    #[token(r"copy")]
    Copy,
    #[token(r"index")]
    Index,
    #[token(r"roll")]
    Roll,
    #[token(r"dup")]
    Dup,
    #[token("pop")]
    Pop,
    #[token("exch")]
    Exch,
    #[token(r"add")]
    Add,
    #[token(r"sub")]
    Sub,
    #[token(r"mul")]
    Mul,
    #[token(r"div")]
    Div,
    #[token(r"mod")]
    Mod,
    #[token(r"def")]
    Def,
    #[token(r"gt")]
    Gt,
    #[token(r"eq")]
    Eq,
    #[token(r"ne")]
    Ne,
    // #[token(r"undef")]
    // Undef,
    // #[token(r"for")]
    // For,
    #[token(r"exec")]
    Exec,
    #[token(r"repeat")]
    Repeat,
    #[token(r"ifelse")]
    IfElse,
    #[token(r"if")]
    If,
    #[regex(r"[\[{]|<<")]
    #[token(r"mark")]
    Mark,
    #[token(r"]")]
    Array,
    #[token(r"}")]
    Proc,
    // #[token(r">>")]
    // Dict,
    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice().to_owned())]
    ExeName(String),
    #[regex(r"//[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[2..].to_owned())]
    ImmName(String),
    #[regex(r"/[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[1..].to_owned())]
    LitName(String),
}

pub fn get_action(token: &Token) -> Action {
    match token {
        Token::Add => Action::ExecBuiltin(Builtin::Add),
        Token::Array => Action::MakeArray,
        Token::Bool(b) => Action::Push(Item::Bool(*b)),
        Token::Copy => Action::ExecBuiltin(Builtin::Copy),
        Token::Index => Action::ExecBuiltin(Builtin::Index),
        Token::Clear => Action::ExecBuiltin(Builtin::Clear),
        Token::ClearToMark => Action::ClearToMark,
        Token::CountToMark => Action::CountToMark,
        Token::Def => Action::ExecBuiltin(Builtin::Def),
        Token::Div => Action::ExecBuiltin(Builtin::Div),
        Token::Exec => Action::ExecBuiltin(Builtin::Exec),
        Token::PopAndPrint => Action::ExecBuiltin(Builtin::PopAndPrint),
        Token::Dup => Action::ExecBuiltin(Builtin::Dup),
        Token::Eq => Action::ExecBuiltin(Builtin::Eq),
        Token::Exch => Action::ExecBuiltin(Builtin::Exch),
        Token::ExeName(n) => Action::ExecName(n.clone()),
        Token::Gt => Action::ExecBuiltin(Builtin::Gt),
        Token::Ne => Action::ExecBuiltin(Builtin::Ne),
        Token::If => Action::ExecBuiltin(Builtin::If),
        Token::IfElse => Action::ExecBuiltin(Builtin::IfElse),
        Token::Integer(i) => Action::Push(Item::Integer(*i)),
        Token::LitName(n) => Action::Push(Item::LitName(n.clone())),
        Token::ImmName(n) => Action::PushImmName(n.clone()),
        Token::Mark => Action::Push(Item::Mark),
        Token::Mod => Action::ExecBuiltin(Builtin::Mod),
        Token::Mul => Action::ExecBuiltin(Builtin::Mul),
        Token::Pop => Action::ExecBuiltin(Builtin::Pop),
        Token::Proc => Action::MakeProc,
        Token::Repeat => Action::ExecBuiltin(Builtin::Repeat),
        Token::Roll => Action::ExecBuiltin(Builtin::Roll),
        Token::Sub => Action::ExecBuiltin(Builtin::Sub),
        Token::Stack => Action::Stack,
    }
}
