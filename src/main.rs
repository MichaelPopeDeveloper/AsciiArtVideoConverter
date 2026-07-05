mod ascii;
mod decode;
mod playback;

use playback::play;

pub fn main() -> anyhow::Result<()> {
    let path = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        "loser.mp4".to_string()
    };
    let (cols, _rows) = crossterm::terminal::size()?;
    play(&path, cols as u32)?;
    Ok(())
}