use std::time::Instant;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Timer {
    worktime: f64,
    breaktime: f64,
    status: Arc<Mutex<Status>>,
    
}
impl Timer {
    pub fn new_timer( worktime: f64, breaktime:  f64, status: Arc<Mutex<Status>>) -> Timer {
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

#[derive(Clone)]
pub struct WorkTimer {
    worktime: f64,
    breaktime: f64,
    status: Arc<Mutex<Status>>,
}

impl WorkTimer {

    pub fn start_timer(worktime: f64, breaktime: f64, status: Arc<Mutex<Status>>) -> Option<WorkTimer>{

        let timer =  WorkTimer{
            worktime,
            breaktime,
            status,
            };
            let elapsed = Instant::now();
    
            while elapsed.elapsed().as_secs_f64() < timer.worktime {

                if *timer.status.lock().unwrap() == Status::Active {

                    stdout().flush().unwrap();
                    print!("\rWork time left {:.2?}", timer.worktime -  elapsed.elapsed().as_secs_f64());

                } else if *timer.status.lock().unwrap() == Status::Pause {
 
                    loop {
                       
                        stdout().flush().unwrap();
                        print!("\rWork timer paused at {:.2?}", timer.worktime - elapsed.elapsed().as_secs_f64());
                         // failure to pause printed time problem is here
                        if *timer.status.lock().unwrap() == Status::Active {
                           
                            break;
                        } 
                    }
                }
                
            };
    
            Some(timer)

    }

    pub fn start_break(self) -> Option<BreakTimer> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }

    
}

#[derive(Clone)]
pub struct BreakTimer {
    worktime:  f64,
    breaktime:  f64,
    status: Arc<Mutex<Status>>,
}

impl BreakTimer {
    pub fn start_timer(worktime:  f64, breaktime:  f64, status: Arc<Mutex<Status>>) -> Option<BreakTimer> {
        let timer =  BreakTimer{
        worktime,
        breaktime,
        status,
        };
        let elapsed = Instant::now();

        while elapsed.elapsed().as_secs_f64() < timer.breaktime {

            if *timer.status.lock().unwrap() == Status::Active {

                stdout().flush().unwrap();
                print!("\rBreak time left {:.2?}", timer.breaktime -  elapsed.elapsed().as_secs_f64());
            
            
            } else if *timer.status.lock().unwrap() == Status::Pause {

                

                loop {
                    stdout().flush().unwrap();
                    print!("\rBreak timer paused at {:.2?}", timer.breaktime - elapsed.elapsed().as_secs_f64());
                        // failure to pause printed time problem is here
                    if *timer.status.lock().unwrap() == Status::Active {
                        
                        break;
                    } 
                }
            };
        };

        Some(timer)

    }
    pub fn start_work(self) -> Option<WorkTimer> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime, self.status);
        timer
    }
}

#[derive(PartialEq, Clone)]
pub enum Status {
    Pause,
    Reset,
    Stop,
    Active,
}












#[cfg(test)]

mod tests {
  





    
}