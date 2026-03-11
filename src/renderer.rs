use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Instant;
use terminal_size::{Height, Width, terminal_size};

type AtomicPixel = (AtomicU8, AtomicU8, AtomicU8);
type Pixel = (u8, u8, u8);

pub fn render(sigma: Arc<Vec<Vec<AtomicPixel>>>) -> std::io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[?25l");

    let (sigma_height, sigma_width) = (sigma.len(), sigma[0].len());
    assert!(sigma_height < 256 && sigma_width < 256);
    let total_rows = sigma_height.div_ceil(2);
    let num_pairs = sigma_height / 2;
    let mut prev_pairs: Vec<Vec<(Pixel, Pixel)>> = Vec::with_capacity(num_pairs);
    for i in (0..num_pairs * 2).step_by(2) {
        let mut row_vec = Vec::with_capacity(sigma_width);
        for j in 0..sigma_width {
            let top = (
                sigma[i][j].0.load(Ordering::Relaxed),
                sigma[i][j].1.load(Ordering::Relaxed),
                sigma[i][j].2.load(Ordering::Relaxed),
            );
            let bottom = (
                sigma[i + 1][j].0.load(Ordering::Relaxed),
                sigma[i + 1][j].1.load(Ordering::Relaxed),
                sigma[i + 1][j].2.load(Ordering::Relaxed),
            );
            row_vec.push((top, bottom));
        }
        prev_pairs.push(row_vec);
    }

    let mut prev_last_row: Option<Vec<(u8, u8, u8)>> = if sigma_height % 2 == 1 {
        let last_idx = sigma_height - 1;
        let mut row_vec = Vec::with_capacity(sigma_width);
        for j in 0..sigma_width {
            let fg = (
                sigma[last_idx][j].0.load(Ordering::Relaxed),
                sigma[last_idx][j].1.load(Ordering::Relaxed),
                sigma[last_idx][j].2.load(Ordering::Relaxed),
            );
            row_vec.push(fg);
        }
        Some(row_vec)
    } else {
        None
    };

    loop {
        let frame_start = Instant::now();
        if let Some((Width(w), Height(h))) = terminal_size() {
            if h < total_rows as u16 || w < sigma_width as u16 {
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "Sigma is too big for the current terminal size",
                ));
            };
        } else {
            return Err(io::Error::other("Could not measure terminal"));
        }

        let mut buf = Vec::<u8>::with_capacity((sigma_height / 2) * sigma_width * 30);

        buf.extend_from_slice(b"\x1B[?2026h");

        for (row, row_pair) in prev_pairs.iter_mut().enumerate() {
            let upper_row = 2 * row;
            let lower_row = upper_row + 1;
            for col in 0..sigma_width {
                let cur_top = (
                    sigma[upper_row][col].0.load(Ordering::Relaxed),
                    sigma[upper_row][col].1.load(Ordering::Relaxed),
                    sigma[upper_row][col].2.load(Ordering::Relaxed),
                );
                let cur_bottom = (
                    sigma[lower_row][col].0.load(Ordering::Relaxed),
                    sigma[lower_row][col].1.load(Ordering::Relaxed),
                    sigma[lower_row][col].2.load(Ordering::Relaxed),
                );
                let prev = &mut row_pair[col];
                if (cur_top, cur_bottom) != *prev {
                    *prev = (cur_top, cur_bottom);
                    write_cursor_move(&mut buf, row as u8 + 1, col as u8 + 1);
                    write_colors(&mut buf, cur_top, cur_bottom);
                    buf.extend_from_slice(b"\xE2\x96\x80");
                }
            }
        }

        if let Some(prev_row) = &mut prev_last_row {
            let last_idx = sigma_height - 1;
            let row = (num_pairs + 1) as u8;
            for col in 0..sigma_width {
                let cur_fg = (
                    sigma[last_idx][col].0.load(Ordering::Relaxed),
                    sigma[last_idx][col].1.load(Ordering::Relaxed),
                    sigma[last_idx][col].2.load(Ordering::Relaxed),
                );
                if cur_fg != prev_row[col] {
                    prev_row[col] = cur_fg;
                    write_cursor_move(&mut buf, row, (col as u8) + 1);
                    buf.extend_from_slice(b"\x1B[38;2;");
                    write_u8(&mut buf, cur_fg.0);
                    buf.push(b';');
                    write_u8(&mut buf, cur_fg.1);
                    buf.push(b';');
                    write_u8(&mut buf, cur_fg.2);
                    buf.extend_from_slice(b";49m");
                    buf.extend_from_slice(b"\xE2\x96\x80");
                }
            }
        }

        let status_line = total_rows as u8 + 1;
        write_cursor_move(&mut buf, status_line, 1);
        buf.extend_from_slice(b"\x1B[0m");
        buf.extend_from_slice(b"\x1B[2K");
        write!(buf, "Frame Time: {:?}", frame_start.elapsed())?;
        buf.extend_from_slice(b"\x1B[0m");
        buf.extend_from_slice(b"\x1B[?2026l");

        io::stdout().write_all(&buf)?;
        io::stdout().flush()?;
    }
}

#[inline(always)]
fn write_u8(buf: &mut Vec<u8>, mut n: u8) {
    if n >= 100 {
        buf.push(b'0' + n / 100);
        n %= 100;
        buf.push(b'0' + n / 10);
        buf.push(b'0' + n % 10);
    } else if n >= 10 {
        buf.push(b'0' + n / 10);
        buf.push(b'0' + n % 10);
    } else {
        buf.push(b'0' + n);
    }
}

#[inline(always)]
fn write_cursor_move(buf: &mut Vec<u8>, row: u8, col: u8) {
    buf.extend_from_slice(b"\x1B[");
    write_u8(buf, row);
    buf.push(b';');
    write_u8(buf, col);
    buf.push(b'H');
}

#[inline(always)]
fn write_colors(buf: &mut Vec<u8>, fg: Pixel, bg: Pixel) {
    buf.extend_from_slice(b"\x1B[38;2;");
    write_u8(buf, fg.0);
    buf.push(b';');
    write_u8(buf, fg.1);
    buf.push(b';');
    write_u8(buf, fg.2);
    buf.extend_from_slice(b";48;2;");
    write_u8(buf, bg.0);
    buf.push(b';');
    write_u8(buf, bg.1);
    buf.push(b';');
    write_u8(buf, bg.2);
    buf.push(b'm');
}
