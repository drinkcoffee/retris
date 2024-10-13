mod state;
use state::{
    fall_one, move_left, move_right, rotate_anticlockwise, rotate_clockwise, start_new_four,
    update_if_dropped,
};

mod render;
use render::render;

use std::io;

use std::time::Duration;

use crossterm::{
    event::{poll, read, /*DisableMouseCapture, EnableMouseCapture, */ Event, KeyCode},
    /* execute, */
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    start_new_four();

    loop {
        fall_one();
        update_if_dropped();
        render().unwrap();

        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if event == Event::Key(KeyCode::Left.into()) {
                move_left();
            }
            if event == Event::Key(KeyCode::Right.into()) {
                move_right();
            }
            if event == Event::Key(KeyCode::Char('z').into()) {
                rotate_anticlockwise();
            }
            if event == Event::Key(KeyCode::Char('x').into()) {
                rotate_clockwise();
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
        }
    }

    disable_raw_mode()?;

    Ok(())
}
