use std::time::SystemTime;
use std::io::{stdout, Write};

#[derive(Copy, Clone)]
pub struct Timer {
    worktime: f64,
    breaktime: f64,
}
impl Timer {
    pub fn new_timer( worktime: f64, breaktime:  f64) -> Timer {
        let timer = Timer {
        worktime,
        breaktime,
        };
        timer
    }
    pub fn start_break(self) -> Option<BreakTimer> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime);
        timer
    }

    pub fn start_work(self) -> Option<WorkTimer> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime);
        timer
    }
}

#[derive(Copy, Clone)]
pub struct WorkTimer {
    worktime: f64,
    breaktime: f64,
}

impl WorkTimer {

    pub fn start_timer(worktime: f64, breaktime: f64) -> Option<WorkTimer>{

        let timer =  WorkTimer{
            worktime,
            breaktime,
            };
            let elapsed = SystemTime::now();
    
            while elapsed.elapsed().unwrap().as_secs_f64() < timer.worktime {
                stdout().flush().unwrap();
                print!("\rWork time left {:.2?}", timer.worktime -  elapsed.elapsed().unwrap().as_secs_f64());
                
            };
    
            Some(timer)

    }

    pub fn start_break(self) -> Option<BreakTimer> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime);
        timer
    }

    
}

#[derive(Copy, Clone)]
pub struct BreakTimer {
    worktime:  f64,
    breaktime:  f64,
}

impl BreakTimer {
    pub fn start_timer(worktime:  f64, breaktime:  f64) -> Option<BreakTimer> {
        let timer =  BreakTimer{
        worktime,
        breaktime,
        };
        let elapsed = SystemTime::now();

        while elapsed.elapsed().unwrap().as_secs_f64() < timer.breaktime {
            stdout().flush().unwrap();
            print!("\rBreak time left {:.2?}", timer.breaktime -  elapsed.elapsed().unwrap().as_secs_f64());
        };

        Some(timer)

    }
    pub fn start_work(self) -> Option<WorkTimer> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime);
        timer
    }
}












#[cfg(test)]

mod tests {
  





    
}