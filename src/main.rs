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
    let width = 25;
    let height = 25;
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
        loop {
            let snapshot: Vec<Vec<Pixel>> = render_sigma
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|(r, g, b)| {
                            (
                                r.load(Ordering::Relaxed),
                                g.load(Ordering::Relaxed),
                                b.load(Ordering::Relaxed),
                            )
                        })
                        .collect()
                })
                .collect();
            let _ = renderer::render(&snapshot);
            thread::sleep(Duration::from_millis(100)); // Cap fps
        }
    });
    let mut rng = rand::rng();
    loop {
        for y in 0..height {
            for x in 0..width {
                // Get a reference to the atomic tuple at (x, y)
                let (ref r, ref g, ref b) = sigma[y][x];
                r.store(rng.random(), Ordering::Relaxed);
                g.store(rng.random(), Ordering::Relaxed);
                b.store(rng.random(), Ordering::Relaxed);
            }
        }
    }
}
