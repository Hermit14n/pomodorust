use pomodorust::{BreakTimer, WorkTimer, Timer};



fn main() {
    let worktime = 8.0;
    let breaktime = 10.0;
    let timer = Timer::new_timer(worktime, breaktime);
    let _worktimer = timer.start_work().unwrap();
    loop {
        
        let _breaktimer: BreakTimer = _worktimer.start_break().unwrap();
        let _worktimer: WorkTimer = _breaktimer.start_work().unwrap();
    }
    

    // Want to accept args
        // work time             --wt <f32>
        // break time            --bt <f32>
        // progress bar yes/no   --pb <bool>
        // pretty view           --pv <bool>
}


