use crate::Object;
use crate::ObjectMode::*;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"%.*")]
pub enum Token {
    #[regex(r"[-+]?[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    #[regex(r"[-+]?[0-9]+\.", |lex| lex.slice().parse().ok())]
    #[regex(r"[-+]?\.[0-9]+", |lex| lex.slice().parse().ok())]
    Real(f64),
    #[regex(r"[-+]?[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Bool(bool),
    #[regex(r"\[|<<")]
    #[token(r"mark")]
    Mark,
    #[token(r">>")]
    Dict,
    #[token(r"{")]
    BeginProc,
    #[token(r"}")]
    EndProc,
    #[regex(r"//[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[2..].to_owned())]
    ImmName(String),
    #[regex(r"/[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice()[1..].to_owned())]
    LitName(String),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*", |lex| lex.slice().to_owned())]
    #[token(r"=", |lex| lex.slice().to_owned())]
    #[token(r"==", |lex| lex.slice().to_owned())]
    #[regex(r"\]|>>", |lex| lex.slice().to_owned())]
    ExeName(String),
}

impl Token {
    pub fn to_object(&self) -> Object {
        match self {
            Token::Bool(b) => Object::Bool(*b),
            Token::Real(r) => Object::Real(*r),
            Token::Integer(i) => Object::Integer(*i),
            Token::Mark => Object::Mark,
            Token::ExeName(n) => Object::Name(Executable, n.clone()),
            Token::LitName(n) => Object::Name(Literal, n.clone()),
            _ => panic!("Token not expected {:?}", self),
        }
    }
}
