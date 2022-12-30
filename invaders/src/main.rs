use std::error::Error;
use rusty_audio::Audio;
use std:: {io, thread};
use std::sync::mpsc::{self, Receiver};
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::{event::{self, Event, KeyCode}};
use std::time::{Duration, Instant};

use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    render
};


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


    // Render loop in a separate thread

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, true);
            last_frame = curr_frame;
        }
    });

    // Game loop
    'gameloop: loop {
        // Per frame init 
        let curr_frame = new_frame();
        //input event
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()?{
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
