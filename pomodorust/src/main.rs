use clap::Parser;
use crossterm::{cursor, execute, terminal};
use pomodorust::{BreakTimer, State, Status, Timer, WorkTimer};
use std::io::{stdin, stdout};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
mod args;
mod tui;
use crate::args::TimerArgs;
use crate::tui::tui;

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
    tui(rx, &stdout, state, time_left)?;

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
