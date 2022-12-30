use std::error::Error;
use rusty_audio::Audio;
use std::io;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode","../sounds/explode.wav");
    audio.add("lose","../sounds/lose.wav");
    audio.add("move","../sounds/move.wav");
    audio.add("pew","../sounds/pew.wav");
    audio.add("startup","../sounds/startup.wav");
    audio.add("win","../sounds/win.wav");
    audio.play("startup");


    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //clean up
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
