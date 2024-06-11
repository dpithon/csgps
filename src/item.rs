use crate::Builtin;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Item {
    Integer(i64),
    Bool(bool),
    Mark,
    Builtin(Builtin),
    Array(Vec<Item>),
    Proc(Vec<Item>),
    LitName(String),
    ExeName(String),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LitName(n) => write!(f, "LitName({n})"),
            Self::ExeName(n) => write!(f, "ExeName({n})"),
            Self::Integer(i) => write!(f, "Integer({i})"),
            Self::Bool(b) => write!(f, "Bool({b})"),
            Self::Mark => write!(f, "Mark"),
            Self::Builtin(builtin) => write!(f, "{:?}", builtin),
            Self::Array(a) => {
                write!(f, "Array[")?;
                for item in a.iter() {
                    write!(f, "{item},")?;
                }
                write!(f, "]")
            }
            Self::Proc(a) => {
                write!(f, "Proc {{")?;
                for item in a.iter() {
                    write!(f, "{item},")?;
                }
                write!(f, "}}")
            }
        }
    }
}
