use pomodorust::{BreakTimer, WorkTimer, Timer, Status};
use clap::{arg, command};
use std::sync::mpsc;
use std::thread;


fn main() {
    let mut worktime = 8.0;
    let mut breaktime = 10.0;
    let mut rounds: i32 = 10;
    let status = Status{ pause: Some(false), 
                                     reset: Some(false), 
                                     stop: Some(false)
                                    };

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
        ).value_parser(clap::value_parser!(i32)))
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
    let timer = Timer::new_timer(worktime, breaktime, status);
    let handle = thread::spawn(move || {
        let _worktimer = timer.start_work().unwrap();
        tx.send(_worktimer).expect("Problem with channel to worker thread");
        
    loop {
        rounds -= 1;
        if rounds == 0 {
            break
        }
        let _breaktimer: BreakTimer = _worktimer.start_break().unwrap();
        let _worktimer: WorkTimer = _breaktimer.start_work().unwrap();

         } 
    });
    //------------Worker Thread end-----------------//

    handle.join().expect("Thread panicked"); // termination of the main thread will also
                                                  // terminate child thread, join keeps
                                                  // main in scope so child thread can run

    // Want to accept args
        // work time             -w <f64>    Done
        // break time            -b <f64>    Done
        // # rounds              -r <i32>    Done
        // separate worker thread            Done
        // progress bar yes/no   -p <bool>
        // pretty view           -v <bool>
}


