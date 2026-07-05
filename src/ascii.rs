use image::RgbImage;
use image::imageops::FilterType;
const RAMP: &'static [u8; 70] = b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "; // const is static with arrs

fn pixel_to_char(lum: u8) -> char {
    let idx = ((lum as usize) * (RAMP.len() - 1)) / 255;
    // fixed indexing errors with the RAMP const using .as_slice()
    RAMP.as_slice()[(RAMP.len() as usize) - 1 - idx] as char
}

fn gamma(c: u8) -> u8 {
    (255.0 * ((c as f32) / 255.0).powf(0.7)) as u8
}

pub fn frame_to_ascii(frame: &RgbImage, cols: u32) -> String {
    let aspect = (frame.height() as f32) / (frame.width() as f32);
    let rows = ((cols as f32) * aspect * 0.5) as u32;
    let small = image::imageops::resize(frame, cols, rows, FilterType::Lanczos3);

    let mut out = String::with_capacity((cols * rows * 20 + rows * 5) as usize);
    for y in 0..rows {
        for x in 0..cols {
            let p = small.get_pixel(x, y);
            let (r, g, b) = (gamma(p[0]), gamma(p[1]), gamma(p[2]));
            let lum = (0.299 * (r as f32) + 0.587 * (g as f32) + 0.114 * (b as f32)) as u8;
            let ch = pixel_to_char(lum); // brightness -> glyph
            out.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
        }
        out.push_str("\x1b[0m\n");
    }
    out
}
