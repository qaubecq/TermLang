use core::panic;
use std::sync::Arc;

use crate::{AtomicPixel, builtin, kerneler::CodeLine};

#[derive(Debug)]
pub enum Procedure {
    BuiltIn{arg_count: usize, func: fn(&Arc<Vec<Vec<AtomicPixel>>>, Vec<u8>)},
    BuiltOut{arg_count: usize, code: Vec<Instruction>}
}

#[derive(Debug)]
pub enum Instruction {
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


pub fn create_syntax_tree(lines: &Vec<CodeLine>) -> Vec<Procedure> {
    // Create procedure names vec with builtin functions
    let mut proc_names: Vec<&str> = vec!["$write", "$add", "$sub", "$mult", "$div", "$mod", "$eq", "$neq", "$g", "$l", "$geq", "$leq", "$and", "$or", "$xor", "$rsh", "$lsh", "$bonot", "$binot"];

    // Create the Procedure vec
    let mut procs: Vec<Procedure> = vec![
        Procedure::BuiltIn { arg_count: 4, func: builtin::write },
        Procedure::BuiltIn { arg_count: 5, func: builtin::add },
        Procedure::BuiltIn { arg_count: 5, func: builtin::sub },
        Procedure::BuiltIn { arg_count: 5, func: builtin::mult },
        Procedure::BuiltIn { arg_count: 5, func: builtin::div },
        Procedure::BuiltIn { arg_count: 5, func: builtin::modulo },
        Procedure::BuiltIn { arg_count: 5, func: builtin::eq },
        Procedure::BuiltIn { arg_count: 5, func: builtin::neq },
        Procedure::BuiltIn { arg_count: 5, func: builtin::g },
        Procedure::BuiltIn { arg_count: 5, func: builtin::l },
        Procedure::BuiltIn { arg_count: 5, func: builtin::geq },
        Procedure::BuiltIn { arg_count: 5, func: builtin::leq },
        Procedure::BuiltIn { arg_count: 5, func: builtin::and },
        Procedure::BuiltIn { arg_count: 5, func: builtin::or },
        Procedure::BuiltIn { arg_count: 5, func: builtin::xor },
        Procedure::BuiltIn { arg_count: 5, func: builtin::rsh },
        Procedure::BuiltIn { arg_count: 5, func: builtin::lsh },
        Procedure::BuiltIn { arg_count: 4, func: builtin::bonot },
        Procedure::BuiltIn { arg_count: 4, func: builtin::binot }
    ];

    // Go through the code to find all function names
    for line in lines {
        if line.depth == 0 {
            if !line.code.starts_with("proc:") {
                panic!("Parsing Error : Code outside procedures\n | {}", line.code);
            }
            // Get name
            let name = line.code[5..line.code.len()].split('(').next().unwrap();
            proc_names.push(name);
        }
    }

    // Create the tree
    let mut args: Vec<&str> = Vec::new();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.depth == 0 {
            // Get arg names
            args = if let Some(start) = line.code.find('(') {
                let inside = &line.code[start + 1..line.code.len() - 1];
                inside.split(',').collect()
            } else {
                panic!("Parsing Error : Invalid procedure definition\n | {}", line.code);
            };
            // Add proc
            procs.push(Procedure::BuiltOut { arg_count: args.len(), code: Vec::new() });
            // Skip the rest of the iteration
            continue;
        }

        let proc = match procs.last_mut() {
            Some(Procedure::BuiltOut { arg_count, code }) => (arg_count, code),
            _ => panic!("Parsing Error : No procedure initialized\n | {}", line.code)
        };

        // Else
        if line.starts_closure && line.code.starts_with("else") {
            continue;
        }

        // While
        if line.starts_closure && line.code.starts_with("while") {
            let value = Value::new(&line.code.as_str()[6..line.code.len()-1], &args);
            let depth = line.depth;
            let mut next_index = proc.1.len()+1;
            let mut j = i+1;
            while j < lines.len() && lines[j].depth > depth {
                if !(lines[j].starts_closure && lines[j].code.starts_with("else")) {
                    next_index += 1;
                }
                j += 1;
            }
            proc.1.push(Instruction::While { condition: value, next_index : next_index });
        }

        // If
        else if line.starts_closure && line.code.starts_with("if") {
            let value = Value::new(&line.code.as_str()[3..line.code.len()-1], &args);
            let depth = line.depth;
            let mut else_index = proc.1.len()+1;
            let mut found_else = false;
            let mut next_index = proc.1.len()+1;
            let mut j = i+1;
            while j < lines.len() && (lines[j].depth > depth || !found_else) {
                if lines[j].depth==depth && lines[j].starts_closure && lines[j].code.starts_with("else") {
                    found_else = true;
                }
                if !(lines[j].starts_closure && lines[j].code.starts_with("else")) {
                    next_index += 1;
                    if !found_else { else_index += 1; }
                }
                j += 1;
            }
            proc.1.push(Instruction::IfElse { condition: value, else_index : else_index, next_index : next_index });
        }

        // Proc call
        else {
            let name = line.code.split('(').next().unwrap();
            let proc_index = proc_names.iter().position(|n| *n == name).expect(format!("Parsing Error : Procedure doesn't exist\n | {}", line.code).as_str());
            let args = if let Some(start) = line.code.find('(') {
                let inside = &line.code[start + 1..line.code.len() - 1];
                //inside.split(',').map(|s| Value::new(s, &args)).collect()  // WARNING DOES NOT WORK WITH NESTED ,
                // Get args value
                let mut arg_names: Vec<String> = Vec::new();
                let mut buffer: String = String::new();
                let mut depth = 0;
                for char in inside.chars() {
                    if char == '[' {
                        depth += 1;
                    } else if char == ']' {
                        depth -= 1;
                    } else if depth == 0 && char==',' {
                        arg_names.push(std::mem::take(&mut buffer));
                        continue;
                    }
                    buffer.push(char);
                }
                arg_names.push(buffer);
                arg_names.iter().map(|s| Value::new(s, &args)).collect()
            } else {
                panic!("Parsing Error : Invalid procedure call\n | {}", line.code);
            };
            proc.1.push(Instruction::Call { proc_index: proc_index, args: args });
        }
    }


    return procs;
}