use crate::Op;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ObjectMode {
    Literal,
    Executable,
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Real(f64),
    Bool(bool),
    Mark,
    Array(ObjectMode, Vec<Object>),
    Name(ObjectMode, String),
    Operator(ObjectMode, Op),
    String(ObjectMode, String),
    File(ObjectMode, String),
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
