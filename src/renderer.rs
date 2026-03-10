use terminal_size::{terminal_size,Width,Height};
use std::io::{self, Write};

pub fn render(sigma: &Vec<Vec<(u8, u8, u8)>>) -> std::io::Result<()> {
    if sigma.is_empty() {
        return Ok(());
    };
    if let Some((Width(w), Height(h))) = terminal_size() {
        if h < sigma.len() as u16 || w < sigma[0].len() as u16 {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Sigma is too big for the current terminal size",
            ));
        };
    } else {
        return Err(io::Error::other("Could not measure terminal"));
    }

    let mut stdout = std::io::stdout();
    write!(stdout, "\x1B[H")?;
    for lines in sigma.chunks(2) {
        if lines.len() == 2 {
            let upper = &lines[0];
            let lower = &lines[1];
            for (up, down) in upper.iter().zip(lower) {
                write!(
                    stdout,
                    "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m▀",
                    up.0, up.1, up.2, down.0, down.1, down.2,
                )?;
            }
            write!(stdout, "\x1B[E")?;
        } else {
            write!(stdout, "\x1B[49m")?;
            let line = &lines[0];
            for up in line {
                write!(stdout, "\x1B[38;2;{};{};{}m▀", up.0, up.1, up.2,)?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
