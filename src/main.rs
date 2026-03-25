use rand::RngExt;
mod renderer;

use std::{
    sync::Arc,
    sync::atomic::{AtomicU8, Ordering},
    thread,
};

type AtomicPixel = [AtomicU8; 3];

fn main() {
    let width = 255;
    let height = 255;
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
    let mut rng = rand::rng();
    for _ in 0.. {
        for x in 0..height {
            for y in 0..width {
                for z in 0..3 {
                    sigma[x][y][z].store(rng.random(), Ordering::Relaxed)
                }
            }
        }
    }
}
