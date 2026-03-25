use std::sync::Arc;

use crate::{AtomicPixel, builtin};

enum Procedure {
    BuiltIn{arg_count: usize, func: fn(&Arc<Vec<Vec<AtomicPixel>>>, Vec<u8>)},
    BuiltOut{arg_count: usize, code: Vec<Instruction>}
}

enum Instruction {
    Call{proc_index: usize, args: Vec<Value>},
    IfElse{condition: Value, else_index: usize, next_index: usize},
    While{condition: Value, next_index: usize}
}

enum Value {
    Pure{value: usize},
    Argument{index: usize},
    Memory(Box<Value>, Box<Value>, Box<Value>)
}

