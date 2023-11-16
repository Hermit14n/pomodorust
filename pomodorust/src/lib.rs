use std::time::{Duration, SystemTime};
use std::io::Write;
use std::{sync::mpsc, thread};



pub struct Timer<'a> {
    worktime: &'a mut f64,
    breaktime: &'a mut f64,


}

impl<'a> Timer<'a> {
    pub fn new_timer( worktime: &'a mut f64, breaktime: &'a mut f64) -> Timer<'a> {
        let timer = Timer {
        worktime,
        breaktime,
        };
        timer
    }
    pub fn start_break(self) -> BreakTimer<'a> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime);
        timer
    }

    pub fn start_work(self) -> WorkTimer<'a> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime);
        timer
    }
}

pub struct WorkTimer<'a> {
    worktime: &'a mut f64,
    breaktime: &'a mut f64,
    time_left: f64,
    

}

impl<'a> WorkTimer<'a> {

    pub fn start_timer(worktime: &'a mut f64, breaktime: &'a mut f64) -> WorkTimer<'a>{

        let timer =  WorkTimer{
            worktime,
            breaktime,
            time_left: worktime.clone(),
            };
            let elapsed = SystemTime::now();
    
            while &mut elapsed.elapsed().unwrap().as_secs_f64() < timer.breaktime {
                println!("Work time left {:?}", *timer.breaktime -  &elapsed.elapsed().unwrap().as_secs_f64())
            };
    
            timer

    }

    pub fn start_break(self) -> BreakTimer<'a> { // Start break consumes WorkTimer and creates a BreakTimer
        let timer = BreakTimer::start_timer(self.worktime, self.breaktime);
        timer
    }

    
}

pub struct BreakTimer<'a> {
    worktime: &'a mut f64,
    breaktime: &'a mut f64,


}

impl<'a> BreakTimer<'a> {
    pub fn start_timer(worktime: &'a mut f64, breaktime: &'a mut f64) -> BreakTimer<'a> {
        let timer =  BreakTimer{
        worktime,
        breaktime,
        };
        let elapsed = SystemTime::now();

        while &mut elapsed.elapsed().unwrap().as_secs_f64() < timer.breaktime {
            println!("Work time left {:?}", *timer.breaktime -  &elapsed.elapsed().unwrap().as_secs_f64())
        };

        timer

    }
    pub fn start_work(self) -> WorkTimer<'a> { // start work consumes BreakTimer and returns a Worktimer
        let timer = WorkTimer::start_timer(self.worktime, self.breaktime);
        timer
    }
}












#[cfg(test)]

mod tests {
  





    
}