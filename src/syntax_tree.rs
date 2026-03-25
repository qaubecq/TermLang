use core::panic;
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

#[derive(Debug)]
pub enum Value {
    Pure{value: u8},
    Argument{index: usize},
    Memory(Box<Value>, Box<Value>, Box<Value>)
}
impl Value {
    pub fn new(str: &str, arg_names: &Vec<&str>) -> Self {
        if str.chars().all(|c| c.is_ascii_digit()) {
            // Pure number
            return Value::Pure { value: str.parse::<u8>().expect(format!("Value Parse Error : Invalid pure value : {}", str).as_str()) };
        } 
        else if !(str.contains('[') || str.contains(']')) {
            let index = arg_names.iter().position(|&name| name==str).expect(format!("Value Parse Error : Invalid procedure argument : {}", str).as_str());
            return Value::Argument { index };
        } else if str.starts_with('[') && str.ends_with(']') {
            let mut addr: [String; 3] = [String::new(), String::new(), String::new()];
            let mut index = 0;
            let mut depth = 0;
            for char in str.chars().skip(1).take(str.len()-2) {
                if char == '[' {
                    depth += 1;
                } else if char == ']' {
                    depth -= 1;
                } else if depth == 0 && char==',' {
                    index += 1;
                    continue;
                }
                addr[index].push(char);
            }
            println!("{:?}", addr);
            if addr.len() != 3 {
                panic!("Value Parse Error : Invalid memory read : {}", str);
            }
            return Value::Memory(Box::new(Self::new(addr[0].as_str(), arg_names)), Box::new(Self::new(addr[1].as_str(), arg_names)), Box::new(Self::new(addr[2].as_str(), arg_names)));
        } 
        else {
            panic!("Value Parse Error : Invalid value format : {}", str);
        }
    }

    pub fn eval(&self, sigma: &Arc<Vec<Vec<AtomicPixel>>>, args_value: &Vec<u8>) -> u8 {
        return match self {
            Value::Pure{value} => *value,
            Value::Argument { index } => args_value[*index],
            Value::Memory(one, two, three) => sigma[Value::eval(&**one, sigma, args_value) as usize][Value::eval(&**two, sigma, args_value) as usize][Value::eval(&**three, sigma, args_value) as usize].load(std::sync::atomic::Ordering::Relaxed)
        };
    }
}
