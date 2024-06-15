use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Op {
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
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "--add--"),
            Op::Load => write!(f, "--add--"),
            Op::Clear => write!(f, "--clear--"),
            Op::Copy => write!(f, "--copy--"),
            Op::Index => write!(f, "--index--"),
            Op::Def => write!(f, "--def--"),
            Op::Div => write!(f, "--div--"),
            Op::Exec => write!(f, "--exec--"),
            Op::PopAndPrint => write!(f, "--=--"),
            Op::Dup => write!(f, "--dup--"),
            Op::Eq => write!(f, "--eq--"),
            Op::Ne => write!(f, "--ne--"),
            Op::Exch => write!(f, "--exch--"),
            Op::Gt => write!(f, "--gt--"),
            Op::If => write!(f, "--if--"),
            Op::IfElse => write!(f, "--ifElse--"),
            Op::Mod => write!(f, "--mod--"),
            Op::Mul => write!(f, "--mul--"),
            Op::Pop => write!(f, "--pop--"),
            Op::Repeat => write!(f, "--repeat--"),
            Op::Roll => write!(f, "--roll--"),
            Op::Sub => write!(f, "--sub--"),
            Op::EndArray => write!(f, "--]--"),
            Op::Pstack => write!(f, "--pstack--"),
        }
    }
}
