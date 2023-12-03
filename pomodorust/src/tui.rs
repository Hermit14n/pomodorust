use crate::mpsc::Receiver;
use crate::seconds_to_minutes::format_time;
use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use pomodorust::State;
use std::io::{Stdout, Write};
use std::sync::{Arc, Mutex};

pub fn tui(
    rx: Receiver<()>,
    mut stdout: &Stdout,
    state: Arc<Mutex<State>>,
    time_left: Arc<Mutex<f64>>,
) -> Result<(), std::io::Error> {
    execute!(stdout, cursor::SavePosition)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let mut time_string: String;
    loop {
        for y in 0..21 {
            for x in 0..61 {
                if (y == 0 || y == 21 - 1) || (x == 0 || x == 61 - 1) {
                    // in this loop we are more efficient by not flushing the buffer.

                    queue!(
                        stdout,
                        cursor::MoveTo(x, y),
                        style::PrintStyledContent("█".dark_cyan()),
                        cursor::Hide,
                        //terminal::Clear(terminal::ClearType::UntilNewLine),
                    )?;
                    if x == 61 || (y > 0 && y < 5) || (y > 11 && y < 20) || y > 20 {
                        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
                    }
                }
            }
        }
        time_string = format_time(*time_left.lock().unwrap());
        queue!(
            stdout,
            cursor::MoveTo(5, 8),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(60, 8),
            style::PrintStyledContent("█".dark_cyan()),
            cursor::Hide,
            cursor::MoveTo(5, 9),
            style::PrintStyledContent("Status: ".dark_cyan()),
            style::Print(state.lock().unwrap().to_string()),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(60, 9),
            style::PrintStyledContent("█".dark_cyan()),
            cursor::Hide,
            cursor::MoveTo(5, 10),
            // TODO: seconds don't display 0 before single
            style::PrintStyledContent("Time Left: ".dark_cyan()),
            style::PrintStyledContent(time_string.white()),
          
            // terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(60, 10),
            style::PrintStyledContent("█".dark_cyan()),
            cursor::Hide,
            cursor::MoveTo(5, 11),
            style::PrintStyledContent(
                "Press [p] to pause, [c] to continue, [e] to exit".dark_cyan()
            ),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(60, 11),
            style::PrintStyledContent("█".dark_cyan()),
            cursor::Hide,
        )?;

        if rx.try_recv().is_ok() {
            break;
        }

        stdout.flush()?;
    }
    Ok(())
}
