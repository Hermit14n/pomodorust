use clap::Parser;
use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use pomodorust::{BreakTimer, State, Status, Timer, WorkTimer};
use std::io::{stdin, stdout, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
mod args;
use crate::args::TimerArgs;

fn main() -> std::io::Result<()> {
    let status = Arc::new(Mutex::new(Status::Active));
    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::Stopped));
    let time_left: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));

    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let args = TimerArgs::parse();

    let worktime = args.worktime * 60.0;

    let breaktime = args.breaktime * 60.0;

    let mut rounds = args.rounds;

    //------------Worker Thread init----------------//
    let timer = Timer::new_timer(
        worktime,
        breaktime,
        Arc::clone(&status),
        Arc::clone(&state),
        Arc::clone(&time_left),
    );
    thread::spawn(move || {
        let _worktimer = timer.start_work().unwrap();

        loop {
            rounds -= 1;
            if rx1.try_recv().is_ok() {
                break;
            }
            if rounds == 0 {
                break;
            }
            let _breaktimer: BreakTimer = _worktimer.clone().start_break().unwrap();
            let _worktimer: WorkTimer = _breaktimer.clone().start_work().unwrap();
        }
    });
    //------------Worker Thread end-----------------//

    //------------Input Thread Begin-----------------//
    std::thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Enter valid input");

        buffer = buffer.trim().to_string();

        if buffer == "c" && *status.lock().unwrap() != Status::Active {
            *status.lock().unwrap() = Status::Active;
            continue;
        } else if buffer == "p" {
            *status.lock().unwrap() = Status::Pause;
        } else if buffer == "e" {
            let _ = tx.send(());
            let _ = tx1.send(());
            break;
        }
    });

    //------------Input Thread End-----------------//
    let mut stdout = stdout();
    execute!(stdout, cursor::SavePosition)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
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
                    if x == 60 || (y > 11 && y < 20) || y > 20 {
                        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine),)?;
                    }
                }
            }
        }
        queue!(
            stdout,
            cursor::MoveTo(5, 9),
            style::PrintStyledContent("Status: ".dark_cyan()),
            style::Print(state.lock().unwrap().to_string()),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(60, 9),
            style::PrintStyledContent("█".dark_cyan()),
            cursor::Hide,
            cursor::MoveTo(5, 10),
            style::PrintStyledContent("Time Left: ".dark_cyan()),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            style::PrintStyledContent((*time_left.lock().unwrap().to_string()).white()),
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

    // Want to accept args               Done
    // work time             -w <f64>    Done
    // break time            -b <f64>    Done
    // # rounds              -r <i32>    Done
    // separate worker thread            Done
    // progress bar yes/no   -p <bool>
    // pretty view           -v <bool>   Done, Default
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, cursor::RestorePosition)?;
    Ok(())
}
