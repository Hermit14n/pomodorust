use clap::{arg, command};
use crossterm::{
    cursor, event, execute, queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal, ExecutableCommand,
};
use pomodorust::{BreakTimer, State, Status, Timer, WorkTimer};
use std::io::{stdin, stdout, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() -> std::io::Result<()> {
    let mut worktime: f64 = 8.0;
    let mut breaktime: f64 = 10.0;
    let mut rounds: i32 = 10;
    let status = Arc::new(Mutex::new(Status::Active));
    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::Stopped));
    let time_left: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));

    let (tx, _rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    let matches = command!()
        .arg(
            arg!(
                -w --worktime <f64>... "Add work length in minutes"
            )
            .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(
                -b --breaktime <f64> ... "Add break length in minutes"
            )
            .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(
                -r --rounds <i32> ... "Add number of work/break rounds"
            )
            .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    if let Some(cli_worktime) = matches.get_one::<f64>("worktime") {
        worktime = *cli_worktime * 60.0;
    }

    if let Some(cli_breaktime) = matches.get_one::<f64>("breaktime") {
        breaktime = *cli_breaktime * 60.0;
    }

    if let Some(cli_rounds) = matches.get_one::<i32>("rounds") {
        rounds = *cli_rounds;
    }

    //------------Worker Thread init----------------//
    let timer = Timer::new_timer(
        worktime,
        breaktime,
        Arc::clone(&status),
        Arc::clone(&state),
        Arc::clone(&time_left),
    );
    let handle = thread::spawn(move || {
        // is this actually multithreading?
        // do I need recv?
        let _worktimer = timer.start_work().unwrap();
        tx.send(_worktimer.clone())
            .expect("Problem with channel to worker thread");

        loop {
            rounds -= 1;
            if rounds == 0 {
                break;
            }
            let _breaktimer: BreakTimer = _worktimer.clone().start_break().unwrap();
            let _worktimer: WorkTimer = _breaktimer.clone().start_work().unwrap();
        }
    });
    //------------Worker Thread end-----------------//

    //------------Input Thread Begin-----------------//
    let handle1 = std::thread::spawn(move || {
        tx1.send(status.clone()).expect("Input thread panicked");
        loop {
            
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("Enter valid input");

            buffer = buffer.trim().to_string();

            if buffer == "c" && *status.lock().unwrap() != Status::Active {
                *status.lock().unwrap() = Status::Active;
                continue;
            } else if buffer == "p" {
                *status.lock().unwrap() = Status::Pause;
            } else if buffer == "e" {
                break;
            }
            
        }
    });
   // handle1.join().expect("Input join panicked");
    //------------Input Thread End-----------------//
    let mut stdout = stdout();
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
                    if x == 60 || (y > 11 && y < 20) || y > 20{
                        queue!(
                            stdout,
                            terminal::Clear(terminal::ClearType::UntilNewLine),
                        )?;
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
           
     
           
        

        stdout.flush()?;

        

    }

    handle.join().expect("Thread panicked"); // termination of the main thread will also
                                             // terminate child thread, join keeps
                                             // main in scope so child thread can run

    // Want to accept args               Done
    // work time             -w <f64>    Done
    // break time            -b <f64>    Done
    // # rounds              -r <i32>    Done
    // separate worker thread            Done
    // progress bar yes/no   -p <bool>
    // pretty view           -v <bool>
    Ok(())
}

