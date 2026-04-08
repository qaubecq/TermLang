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

const VERBOSE: bool = true;

fn main() {
    let mut renderer_active = true;
    // Open and files
    let mut contents = String::new();
    let args = env::args().collect::<Vec<String>>();
    for arg in args.iter().skip(1) {
        if arg == "--no_renderer" {
            renderer_active = false;
            continue;
        }
        let mut file = File::open(arg).expect("Failed to open file");
        file.read_to_string(&mut contents).expect("Failed to read file");
    }

    // Parse to kernel language
    let (lines, size) = kernel(contents);
    if VERBOSE {
        println!("Sigma size : {} x {}\n", size[0], size[1]);
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
    if renderer_active {
        let render_sigma = Arc::clone(&sigma);
        thread::spawn(move || {
            renderer::render(render_sigma).unwrap();
        });
    }
    
    // Create syntax tree
    let (tree, main_index) = create_syntax_tree(&lines);

    // Ruuun
    interpret(tree, main_index, sigma);

    // Sleep 0.5 seconds to let render time to render last state
    thread::sleep(Duration::from_millis(500));
}
