use std::env;
use std::time::Duration;
use std::{fs::File, io::Read};
use std::{
    sync::Arc,
    sync::atomic::AtomicU8,
    thread,
};

mod kerneler;
mod syntax_tree;
mod builtin;
mod interpreter;
mod renderer;

use crate::interpreter::interpret;
use crate::kerneler::{format_kernel, kernel};
use crate::syntax_tree::{create_syntax_tree};

type AtomicPixel = [AtomicU8; 3];

const VERBOSE: bool = false;

fn main() {
    // Open file
    let mut file = File::open(&env::args().collect::<Vec<String>>()[1]).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse to kernel language
    let (lines, size) = kernel(contents);
    println!("Sigma size : {} x {}\n", size[0], size[1]);
    if VERBOSE {
        println!("{}", format_kernel(&lines));
    }

    // Create sigma
    let width = size[0];
    let height = size[1];
    let sigma: Arc<Vec<Vec<AtomicPixel>>> = Arc::new(
        (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| [AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0)])
                    .collect()
            })
            .collect(),
    );
    let render_sigma = Arc::clone(&sigma);
    thread::spawn(move || {
        renderer::render(render_sigma).unwrap();
    });
    
    // Create syntax tree
    let (tree, main_index) = create_syntax_tree(&lines);

    // Ruuun
    interpret(tree, main_index, sigma);
    thread::sleep(Duration::from_secs(5));
}
