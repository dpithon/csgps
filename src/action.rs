use crate::{Builtin, Item};

pub enum Action {
    Push(Item),
    PushImmName(String),
    ExecBuiltin(Builtin),
    ClearToMark,
    CountToMark,
    ExecName(String),
    MakeArray,
    MakeProc,
    Stack,
}
