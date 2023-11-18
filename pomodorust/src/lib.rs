use std::time::SystemTime;
use std::io::{stdout, Write};

#[derive(Copy, Clone)]
pub struct Timer {
    worktime: f64,
    breaktime: f64,
    status: Status,
    
}
impl Timer {
    pub fn new_timer( worktime: f64, breaktime:  f64, status: Status) -> Timer {
        let timer = Timer {
        worktime,
        breaktime,
        status,
        };
        timer
    }
    pub fn start_break(self) -> Option<BreakTimer> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }

    pub fn start_work(self) -> Option<WorkTimer> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }
}

#[derive(Copy, Clone)]
pub struct WorkTimer {
    worktime: f64,
    breaktime: f64,
    status: Status,
}

impl WorkTimer {

    pub fn start_timer(worktime: f64, breaktime: f64, status: Status) -> Option<WorkTimer>{

        let timer =  WorkTimer{
            worktime,
            breaktime,
            status,
            };
            let mut elapsed = SystemTime::now();
    
            while elapsed.elapsed().unwrap().as_secs_f64() < timer.worktime {

                if timer.status.pause.unwrap() != true {

                    stdout().flush().unwrap();
                    print!("\rWork time left {:.2?}", timer.worktime -  elapsed.elapsed().unwrap().as_secs_f64());

                } else if timer.status.pause.unwrap() == true {

                    let pause_time = elapsed;
                    stdout().flush().unwrap();
                    print!("\rWork timer paused at {:.2?}", timer.worktime -  pause_time.elapsed().unwrap().as_secs_f64());
                    let difference = elapsed.duration_since(pause_time);
                    elapsed -= difference.unwrap();
                }
                
            };
    
            Some(timer)

    }

    pub fn start_break(self) -> Option<BreakTimer> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }

    
}

#[derive(Copy, Clone)]
pub struct BreakTimer {
    worktime:  f64,
    breaktime:  f64,
    status: Status,
}

impl BreakTimer {
    pub fn start_timer(worktime:  f64, breaktime:  f64, status: Status) -> Option<BreakTimer> {
        let timer =  BreakTimer{
        worktime,
        breaktime,
        status,
        };
        let mut elapsed = SystemTime::now();

        while elapsed.elapsed().unwrap().as_secs_f64() < timer.breaktime {

            if timer.status.pause.unwrap() != true {
                
                stdout().flush().unwrap();
                print!("\rBreak time left {:.2?}", timer.breaktime -  elapsed.elapsed().unwrap().as_secs_f64());
            
            
            } else if timer.status.pause.unwrap() == true {

                let pause_time = elapsed;
                stdout().flush().unwrap();
                print!("\rWork timer paused at {:.2?}", timer.worktime -  pause_time.elapsed().unwrap().as_secs_f64());
                let difference = elapsed.duration_since(pause_time);
                elapsed -= difference.unwrap();
            };
        };

        Some(timer)

    }
    pub fn start_work(self) -> Option<WorkTimer> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }
}

#[derive(Copy, Clone)]
pub struct Status {
    pub pause: Option<bool>,
    pub reset: Option<bool>,
    pub stop: Option<bool>,
}












#[cfg(test)]

mod tests {
  





    
}