use std::sync::{Arc,atomic::Ordering};

use crate::AtomicPixel;

// Arg count : 4  (value, $1, $2, $3)
pub fn write(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    //println!("[{},{},{}] := {}", args[1], args[2], args[3], args[0]);
    if args[2] as usize >= sigma.len() || args[1] as usize >= sigma[0].len() || args[3] >= 3 {panic!("Runtime Error : Invalid memory address : [{},{},{}]", args[2], args[1], args[3])}
    sigma[args[2] as usize][args[1] as usize][args[3] as usize].store(args[0], Ordering::Relaxed);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn add(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0].wrapping_add(args[1]), args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn sub(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0].wrapping_sub(args[1]), args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn mult(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0].wrapping_mul(args[1]), args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn div(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]/args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn modulo(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]%args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn eq(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]==args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn neq(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]!=args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn g(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]>args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn l(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]<args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn geq(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]>=args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn leq(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]<=args[1] {1} else {0}, args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn and(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]&args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn or(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]|args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn xor(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]^args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn rsh(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]>>args[1], args[2], args[3], args[4]]);
}

// Arg count : 5 (value1, value2, $1, $2, $3)
pub fn lsh(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![args[0]<<args[1], args[2], args[3], args[4]]);
}

// Arg count : 4 (value, $1, $2, $3)
pub fn bonot(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![if args[0]!=0 {0} else {1}, args[1], args[2], args[3]]);
}

// Arg count : 4 (value, $1, $2, $3)
pub fn binot(sigma: &Arc<Vec<Vec<AtomicPixel>>>, args: Vec<u8>) {
    write(sigma, vec![!args[0], args[1], args[2], args[3]]);
}