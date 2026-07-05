use std::io::{ stdout, Write };
use std::time::{ Duration, Instant };
use crate::ascii::frame_to_ascii;
use crate::decode::decode;

pub fn play(path: &str, cols: u32) -> anyhow::Result<()> {
    let target = Duration::from_millis(33);
    print!("\x1b[2J");
    decode(path, |frame| {
        let t = Instant::now();
        let art = frame_to_ascii(&frame, cols);
        print!("\x1b[H{art}");
        stdout().flush().ok();
        if let Some(rem) = target.checked_sub(t.elapsed()) {
            std::thread::sleep(rem);
        }
    })?;
    Ok(())
}