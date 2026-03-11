use std::{fs::File, io::Read};

use crate::kerneler::kernel;

mod kerneler;

fn main() {
    // Open file
    let mut file = File::open("programs/test.tl").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Parse to kernel language
    kernel(contents);
}