use rand::RngExt;
use std::time::Duration;
mod renderer;

use std::{
    sync::Arc,
    sync::atomic::{AtomicU8, Ordering},
    thread,
};

type AtomicPixel = (AtomicU8, AtomicU8, AtomicU8);
type Pixel = (u8, u8, u8);

fn main() {
    let width = 255;
    let height = 255;
    let sigma: Arc<Vec<Vec<AtomicPixel>>> = Arc::new(
        (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| (AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0)))
                    .collect()
            })
            .collect(),
    );
    let render_sigma = Arc::clone(&sigma);
    thread::spawn(move || {
        renderer::render(render_sigma).unwrap();
    });
    let mut rng = rand::rng();
    for i in 0.. {
        let number = i % (width * height);
        for y in 0..height {
            for x in 0..width {
                let _value = if y * width + x < number { 0 } else { 255 };
                let (ref r, ref g, ref b) = sigma[y][x];
                r.store(rng.random(), Ordering::Relaxed);
                g.store(rng.random(), Ordering::Relaxed);
                b.store(rng.random(), Ordering::Relaxed);
            }
        }
    }
}
