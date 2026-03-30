use std::{fs::File, io::Read};
use std::{
    sync::Arc,
    sync::atomic::{AtomicU8},
};

mod kerneler;
mod syntax_tree;
mod builtin;
mod interpreter;

use crate::interpreter::interpret;
use crate::kerneler::{format_kernel, kernel};
use crate::syntax_tree::{create_syntax_tree};

type AtomicPixel = [AtomicU8; 3];

fn main() {
    // Open file
    let mut file = File::open("programs/test.tl").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse to kernel language
    let (lines, size) = kernel(contents);
    println!("Sigma size : {} x {}\n", size[0], size[1]);

    println!("{}", format_kernel(&lines));

    let sigma: Arc<Vec<Vec<AtomicPixel>>> = Arc::new((0..size[0]).map(|_| {(0..size[1]).map(|_| [AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0)]).collect()}).collect());

    let (tree, main_index) = create_syntax_tree(&lines);

    interpret(tree, main_index, sigma);
}