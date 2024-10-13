use crate::state;

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

const BORDER: u16 = 1;
const MAX_X: u16 = state::MAX_X + BORDER * 2;
const MAX_Y: u16 = state::MAX_Y + BORDER * 2;

pub fn render() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // Draw border
    for y in 0..MAX_Y {
        for iter in 0..BORDER {
            stdout
                .queue(cursor::MoveTo(iter, y))?
                .queue(style::PrintStyledContent("█".magenta()))?;
            stdout
                .queue(cursor::MoveTo(MAX_X - 1 - iter, y))?
                .queue(style::PrintStyledContent("█".magenta()))?;
        }
    }
    for x in 0..MAX_X {
        for iter in 0..BORDER {
            stdout
                .queue(cursor::MoveTo(x, iter))?
                .queue(style::PrintStyledContent("█".magenta()))?;
            stdout
                .queue(cursor::MoveTo(x, MAX_Y - 1 - iter))?
                .queue(style::PrintStyledContent("█".magenta()))?;
        }
    }

    // Draw screen
    for y in 0..MAX_Y - 2 * BORDER {
        for x in 0..MAX_X - 2 * BORDER {
            let locked = state::get_locked(x, y);
            if locked {
                stdout
                    .queue(cursor::MoveTo(x + BORDER, y + BORDER))?
                    .queue(style::PrintStyledContent("X".red()))?;
            }
        }
    }

    let coords = state::get_falling_coords();
    let x1: u16 = coords.0;
    let y1: u16 = coords.1;
    let x2: u16 = coords.2;
    let y2: u16 = coords.3;
    let x3: u16 = coords.4;
    let y3: u16 = coords.5;
    let x4: u16 = coords.6;
    let y4: u16 = coords.7;

    draw_dot(x1, y1).unwrap();
    draw_dot(x2, y2).unwrap();
    draw_dot(x3, y3).unwrap();
    draw_dot(x4, y4).unwrap();

    stdout.flush()?;
    Ok(())
}

fn draw_dot(x: u16, y: u16) -> io::Result<()> {
    let mut stdout = io::stdout();

    if state::check_valid(x, y) {
        stdout
            .queue(cursor::MoveTo(x + BORDER, y + BORDER))?
            .queue(style::PrintStyledContent("X".blue()))?;
    }
    Ok(())
}
