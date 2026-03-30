use std::vec;
use std::{fs::File, io::Read};
use std::{
    sync::Arc,
    sync::atomic::{AtomicU8, Ordering},
    thread,
};

mod kerneler;
mod syntax_tree;
mod builtin;
mod interpreter;

use crate::kerneler::{format_kernel, kernel};
use crate::syntax_tree::{Value, create_syntax_tree};

type AtomicPixel = [AtomicU8; 3];

fn main() {
    // Open file
    let mut file = File::open("programs/test.tl").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse to kernel language
    let (mut lines, size) = kernel(contents);
    println!("Sigma size : {} x {}\n", size[0], size[1]);
    println!("{}", format_kernel(&mut lines));
    println!("{}", format_kernel(&mut kernel(format_kernel(&mut lines)).0) == format_kernel(&mut lines));


    let mut sigma: Arc<Vec<Vec<AtomicPixel>>> = Arc::new((0..5).map(|_| {(0..5).map(|_| [AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0)]).collect()}).collect());

    sigma[2][2][2].store(1, Ordering::Relaxed);
    sigma[0][1][2].store(3, Ordering::Relaxed);
    sigma[4][3][0].store(152, Ordering::Relaxed);
    let args_name = vec!["x", "y"];
    let args_value: Vec<u8> = vec![0, 2];
    let value = Value::new("[4,[x,[2,2,2],y],x]", &args_name); // [4,[0,1,2],0] -> [4, 3, 0] -> 152
    println!("{:?}", value);
    println!("{}", value.eval(&sigma, &args_value));

    println!("{:?}", create_syntax_tree(&lines));


}