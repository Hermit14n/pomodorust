use pomodorust::{BreakTimer, WorkTimer, Timer, Status};
use clap::{arg, command};
use std::io::{stdout, stdin, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event
};


fn main() -> std::io::Result<()> {

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;

    let mut worktime: f64 = 8.0;
    let mut breaktime: f64 = 10.0;
    let mut rounds: i32 = 10;
    let status = Arc::new(Mutex::new(Status::Active));

    let (tx, _rx) = mpsc::channel();

    let matches = command!()
        .arg(arg!(
            -w --worktime <f64>... "Add work length in minutes"
        ).value_parser(clap::value_parser!(f64)))
        .arg(arg!(
            -b --breaktime <f64> ... "Add break length in minutes"
        ).value_parser(clap::value_parser!(f64)))
        .arg(arg!(
            -r --rounds <i32> ... "Add number of work/break rounds"
        ).value_parser(clap::value_parser!(u32)))
        .get_matches();
    
    if let Some(cli_worktime) = matches.get_one::<f64>("worktime") {
        worktime = *cli_worktime * 60.0;
    }

    if let Some(cli_breaktime) = matches.get_one::<f64>("breaktime") {
        breaktime = *cli_breaktime * 60.0;
    }

    if let Some(cli_rounds) = matches.get_one::<i32>("rounds"){
        rounds = *cli_rounds;
    }

    //------------Worker Thread init----------------//
    let timer = Timer::new_timer(worktime, breaktime, Arc::clone(&status));
    let handle = thread::spawn(move || {
        let _worktimer = timer.start_work().unwrap();
        tx.send(_worktimer.clone()).expect("Problem with channel to worker thread");
        
    loop {
        rounds -= 1;
        if rounds == 0 {
            break
        }
        let _breaktimer: BreakTimer = _worktimer.clone().start_break().unwrap();
        let _worktimer: WorkTimer = _breaktimer.clone().start_work().unwrap();

         } 
    });
    //------------Worker Thread end-----------------//
    loop {
        println!("Press [p] to pause, [c] to continue, [e] to exit");

        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Enter valid input");

        buffer = buffer.trim().to_string();

        if buffer == "c" && *status.lock().unwrap() != Status::Active {
            *status.lock().unwrap() = Status::Active;
            continue;
        } else if buffer == "p"{
            *status.lock().unwrap() = Status::Pause;
        } else if buffer == "e" {
            break;
        }
    };


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


