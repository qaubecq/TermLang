use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Instant;
use terminal_size::{Height, Width, terminal_size};

type AtomicPixel = [AtomicU8; 3];
type Pixel = [u8; 3];

fn load_pixel(pixel: &AtomicPixel) -> Pixel {
    [
        pixel[0].load(Ordering::Relaxed),
        pixel[1].load(Ordering::Relaxed),
        pixel[2].load(Ordering::Relaxed),
    ]
}

struct RenderState {
    sigma_height: u8,
    sigma_width: u8,
    pair_rows: Vec<Vec<(Pixel, Pixel)>>,
    last_row: Option<Vec<Pixel>>,
}

impl RenderState {
    fn build_initial(sigma: &[Vec<AtomicPixel>]) -> Self {
        let (sigma_height, sigma_width) = (sigma.len() as u8, sigma[0].len() as u8);

        let num_pairs = sigma_height as usize / 2;
        let pair_rows = vec![
            vec![([255, 255, 255], [255, 255, 255]); sigma_width as usize];
            num_pairs
        ];

        let last_row = if sigma_height % 2 == 1 {
            Some(vec![[255, 255, 255]; sigma_width as usize])
        } else {
            None
        };

        RenderState {
            sigma_height,
            sigma_width,
            pair_rows,
            last_row,
        }
    }

    fn  write_changes(
        &mut self,
        sigma: &[Vec<AtomicPixel>],
        buf: &mut Vec<u8>,
    ) -> std::io::Result<()> {
        let frame_start = Instant::now();
        let total_rows = self.sigma_height.div_ceil(2);
        let num_pairs = self.sigma_height as usize / 2;
        if let Some((Width(w), Height(h))) = terminal_size() {
            if h < total_rows as u16 || w < self.sigma_width as u16 {
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "Sigma is too big for the current terminal size",
                ));
            };
        } else {
            return Err(io::Error::other("Could not measure terminal"));
        }

        buf.clear();

        buf.extend_from_slice(b"\x1B[?2026h");

        for (row, row_pair) in self.pair_rows.iter_mut().enumerate() {
            let upper_row = 2 * row;
            let lower_row = upper_row + 1;
            for col in 0..self.sigma_width as usize {
                let cur_top = load_pixel(&sigma[upper_row][col]);
                let cur_bottom = load_pixel(&sigma[lower_row][col]);
                let prev = &mut row_pair[col];
                if (cur_top, cur_bottom) != *prev {
                    *prev = (cur_top, cur_bottom);
                    write_cursor_move(buf, row as u8 + 1, col as u8 + 1);
                    write_colors(buf, cur_top, cur_bottom);
                    buf.extend_from_slice(b"\xE2\x96\x80");
                }
            }
        }

        if let Some(prev_row) = &mut self.last_row {
            let last_idx = self.sigma_height as usize - 1;
            let row = (num_pairs + 1) as u8;
            for col in 0..self.sigma_width as usize {
                let cur_fg = load_pixel(&sigma[last_idx][col]);
                if cur_fg != prev_row[col] {
                    prev_row[col] = cur_fg;
                    write_cursor_move(buf, row, (col as u8) + 1);
                    buf.extend_from_slice(b"\x1B[38;2;");
                    write_u8(buf, cur_fg[0]);
                    buf.push(b';');
                    write_u8(buf, cur_fg[1]);
                    buf.push(b';');
                    write_u8(buf, cur_fg[2]);
                    buf.extend_from_slice(b";49m");
                    buf.extend_from_slice(b"\xE2\x96\x80");
                }
            }
        }

        let status_line = total_rows as u8 + 1;
        write_cursor_move(buf, status_line, 1);
        buf.extend_from_slice(b"\x1B[0m");
        buf.extend_from_slice(b"\x1B[2K");
        write!(buf, "Frame Time: {:?}", frame_start.elapsed())?;
        buf.extend_from_slice(b"\x1B[0m");
        buf.extend_from_slice(b"\x1B[?2026l");

        Ok(())
    }
}

pub fn render(sigma: Arc<Vec<Vec<AtomicPixel>>>) -> std::io::Result<()> {
    let mut buf = Vec::<u8>::new();
    buf.extend_from_slice(b"\x1B[2J\x1B[1;1H\x1B[?25l");
    io::stdout().write_all(&buf)?;
    io::stdout().flush()?;
    let mut state = RenderState::build_initial(&sigma);

    loop {
        state.write_changes(&sigma, &mut buf)?;
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
    write_u8(buf, fg[0]);
    buf.push(b';');
    write_u8(buf, fg[1]);
    buf.push(b';');
    write_u8(buf, fg[2]);
    buf.extend_from_slice(b";48;2;");
    write_u8(buf, bg[0]);
    buf.push(b';');
    write_u8(buf, bg[1]);
    buf.push(b';');
    write_u8(buf, bg[2]);
    buf.push(b'm');
}
