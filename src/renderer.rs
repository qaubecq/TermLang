use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;
use terminal_size::{Height, Width, terminal_size};

pub fn render(sigma: &[Vec<(u8, u8, u8)>]) -> std::io::Result<()> {
    let start_time = Instant::now();
    if sigma.is_empty() {
        return Ok(());
    };
    let (sigma_height, sigma_width) = (sigma.len(), sigma[0].len());
    if let Some((Width(w), Height(h))) = terminal_size() {
        if h < (sigma_height / 2) as u16 || w < sigma_width as u16 {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Sigma is too big for the current terminal size",
            ));
        };
    } else {
        return Err(io::Error::other("Could not measure terminal"));
    }

    let mut stdout = std::io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());
    write!(buffer, "\x1B[H").unwrap();
    for lines in sigma.chunks(2) {
        if lines.len() == 2 {
            let upper = &lines[0];
            let lower = &lines[1];
            for (up, down) in upper.iter().zip(lower) {
                write!(
                    buffer,
                    "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m▀",
                    up.0, up.1, up.2, down.0, down.1, down.2,
                )
                .unwrap();
            }
            write!(buffer, "\x1B[E").unwrap();
        } else {
            write!(buffer, "\x1B[49m").unwrap();
            let line = &lines[0];
            for up in line {
                write!(buffer, "\x1B[38;2;{};{};{}m▀", up.0, up.1, up.2,).unwrap();
            }
        }
    }

    write!(buffer, "\x1B[0m\nFrame Time: {:?}", start_time.elapsed())?;
    stdout.flush()?;
    Ok(())
}
