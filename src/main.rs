use std::{fs::File, io::Read};

use crate::kerneler::{format_kernel, kernel};

mod kerneler;

fn main() {
    // Open file
    let mut file = File::open("programs/test.tl").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse to kernel language
    let (mut lines, size) = kernel(contents);
    println!("Sigma size : {} x {}\n", size[0], size[1]);
    println!("{}", format_kernel(&mut lines));
}