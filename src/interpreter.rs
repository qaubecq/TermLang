use std::sync::Arc;

use crate::{AtomicPixel, syntax_tree::{Procedure, Instruction}};


struct StackFrame {
    proc_index: usize,
    args: Vec<u8>,
    inst_index: usize,
    closure_stack: Vec<usize>
}


pub fn interpret(tree: Vec<Procedure>, main_index: usize, sigma: Arc<Vec<Vec<AtomicPixel>>>) {
    // Create stack and push main
    let mut stack: Vec<StackFrame> = Vec::new();
    stack.push(StackFrame { proc_index: main_index, args: Vec::new(), inst_index: 0, closure_stack: Vec::new() }); // Args can later be replaced by program args

    // Interpret Loop
    while !stack.is_empty() {
        let proc = match &tree[stack.last().unwrap().proc_index] {
            Procedure::BuiltOut { arg_count, code } => (arg_count, code),
            _ => unreachable!()
        };

        match &proc.1[stack.last().unwrap().inst_index] {
            Instruction::While { condition, next_index } => {
                if condition.eval(&sigma, &stack.last().unwrap().args) == 0 {
                    stack.last_mut().unwrap().inst_index = *next_index;
                    check(&mut stack, &tree);
                } else {
                    let while_index = stack.last().unwrap().inst_index;
                    stack.last_mut().unwrap().closure_stack.push(while_index);
                    stack.last_mut().unwrap().inst_index += 1;
                    check(&mut stack, &tree);
                }
            },
            Instruction::IfElse { condition, else_index, next_index: _ } => {
                if condition.eval(&sigma, &stack.last().unwrap().args) == 0 {
                    stack.last_mut().unwrap().inst_index = *else_index;
                    check(&mut stack, &tree);
                } else {
                    let if_index = stack.last().unwrap().inst_index;
                    stack.last_mut().unwrap().closure_stack.push(if_index);
                    stack.last_mut().unwrap().inst_index += 1;
                    check(&mut stack, &tree);
                }
            },
            Instruction::Call { proc_index, args } => {
                match &tree[*proc_index] {
                    Procedure::BuiltIn { arg_count, func } => {
                        if *arg_count != args.len() { panic!("Runtime Error : Invalid argument count on procedure call {} instead of {}", args.len(), *arg_count); }
                        func(&sigma, args.iter().map(|a| a.eval(&sigma, &stack.last_mut().unwrap().args)).collect());
                        stack.last_mut().unwrap().inst_index += 1;
                        check(&mut stack, &tree);
                    },
                    Procedure::BuiltOut { arg_count, code: _ } => {
                        if *arg_count != args.len() { panic!("Runtime Error : Invalid argument count on procedure call {} instead of {}", args.len(), *arg_count); }
                        let args_value = args.iter().map(|a| a.eval(&sigma, &stack.last_mut().unwrap().args)).collect();
                        stack.push(StackFrame { proc_index: *proc_index, args: args_value, inst_index: 0, closure_stack: Vec::new() });
                        check(&mut stack, &tree);
                    }
                }
            }
        }
    }
}

fn check(stack: &mut Vec<StackFrame>, tree: &Vec<Procedure>) {
    // If stack is empty, return
    if stack.is_empty() { return ; }

    let frame = stack.last().unwrap();stack.last().unwrap();
    let proc = match &tree[frame.proc_index] {
        Procedure::BuiltOut { arg_count, code } => (arg_count, code),
        _ => unreachable!()
    };
    // Check if closure_stack is empty
    if frame.closure_stack.is_empty() {
        // Only check for out of bound
        if frame.inst_index >= proc.1.len() {
            stack.pop();
            if !stack.is_empty() {
                stack.last_mut().unwrap().inst_index += 1;
                check(stack, tree);
            }
        }
        return;
    }

    // If closure_stack is non empty, match for ifelse or while
    let closure_index = *frame.closure_stack.last().unwrap();
    match &proc.1[closure_index] {
        Instruction::While { condition: _, next_index } => {
            if frame.inst_index == *next_index {
                stack.last_mut().unwrap().inst_index = closure_index;
                stack.last_mut().unwrap().closure_stack.pop();
            }
        },
        Instruction::IfElse { condition: _, else_index, next_index } => {
            if frame.inst_index == *else_index {
                stack.last_mut().unwrap().inst_index = *next_index;
                stack.last_mut().unwrap().closure_stack.pop();
                check(stack, tree);
            }
        },
        _ => unreachable!()
    }
}