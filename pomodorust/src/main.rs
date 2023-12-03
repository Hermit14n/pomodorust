use clap::Parser;
mod seconds_to_minutes;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::{cursor, execute, terminal};
use pomodorust::breaktimer::BreakTimer;
use pomodorust::worktimer::WorkTimer;
use pomodorust::{State, Status, Timer};
use pomodorust::breaktimer;
use std::io::stdout;
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
        match read() {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                ..
            })) => buffer = String::from("p"),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                ..
            })) => buffer = String::from("c"),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                ..
            })) => buffer = String::from("e"),
            _ => (),
        };

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
    // progress bar yes/no   -p <bool>
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, cursor::RestorePosition)?;
    Ok(())
}
