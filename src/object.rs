use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Real(f64),
    Bool(bool),
    Mark,
    Array(ObjectMode, Vec<Object>),
    Name(ObjectMode, String),
    Operator(ObjectMode, Operator),
    String(ObjectMode, String),
    File(ObjectMode, String),
}

#[derive(Debug, Clone)]
pub enum ObjectMode {
    Literal,
    Executable,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Load,
    Clear,
    Copy,
    Index,
    Def,
    Div,
    Exec,
    PopAndPrint, // ==
    Dup,
    Eq,
    Ne,
    Exch,
    Gt,
    If,
    IfElse,
    Mod,
    Mul,
    Pop,
    Repeat,
    Roll,
    Sub,
    EndArray, // ]
    Pstack,
    ClearToMark,
    CountToMark,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(_, n) => write!(f, "Name({n})"),
            Self::File(_, n) => write!(f, "File({n})"),
            Self::Integer(i) => write!(f, "Integer({i})"),
            Self::Real(r) => write!(f, "Real({r})"),
            Self::Bool(b) => write!(f, "Bool({b})"),
            Self::Mark => write!(f, "Mark"),
            Self::String(_, s) => write!(f, "{s}"),
            Self::Operator(_, s) => write!(f, "{s}"),
            Self::Array(_, a) => {
                write!(f, "[")?;
                for object in a.iter() {
                    write!(f, "{object},")?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "--add--"),
            Operator::Load => write!(f, "--add--"),
            Operator::Clear => write!(f, "--clear--"),
            Operator::Copy => write!(f, "--copy--"),
            Operator::Index => write!(f, "--index--"),
            Operator::Def => write!(f, "--def--"),
            Operator::Div => write!(f, "--div--"),
            Operator::Exec => write!(f, "--exec--"),
            Operator::PopAndPrint => write!(f, "--=--"),
            Operator::Dup => write!(f, "--dup--"),
            Operator::Eq => write!(f, "--eq--"),
            Operator::Ne => write!(f, "--ne--"),
            Operator::Exch => write!(f, "--exch--"),
            Operator::Gt => write!(f, "--gt--"),
            Operator::If => write!(f, "--if--"),
            Operator::IfElse => write!(f, "--ifElse--"),
            Operator::Mod => write!(f, "--mod--"),
            Operator::Mul => write!(f, "--mul--"),
            Operator::Pop => write!(f, "--pop--"),
            Operator::Repeat => write!(f, "--repeat--"),
            Operator::Roll => write!(f, "--roll--"),
            Operator::Sub => write!(f, "--sub--"),
            Operator::EndArray => write!(f, "--]--"),
            Operator::Pstack => write!(f, "--pstack--"),
            Operator::ClearToMark => write!(f, "--cleartomark--"),
            Operator::CountToMark => write!(f, "--counttomark--"),
        }
    }
}
