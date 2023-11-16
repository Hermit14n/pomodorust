use std::time::{Duration, SystemTime};
use std::io::Write;
use std::{sync::mpsc, thread};



pub struct Timer {
    worktime: u64,
    breaktime: u64,


}

impl Timer {
    pub fn new_timer(&mut self, worktime: u64, breaktime: u64){
        self.worktime = worktime;
        self.breaktime = breaktime;

    }
}

pub struct WorkTimer<'a> {
    worktime: &'a mut f64,
    breaktime: &'a mut f64,
    

}

impl<'a> WorkTimer<'a> {

    pub fn start_timer(&mut self, worktime: &'a mut f64, breaktime: &'a mut f64) {

        self.worktime = worktime;
        self.breaktime = breaktime;

        let elapsed = SystemTime::now();

        while &elapsed.elapsed().unwrap().as_secs_f64() < self.worktime {
            println!("Work time left {:?}", *self.worktime - &elapsed.elapsed().unwrap().as_secs_f64())
        }

        

    }

    pub fn start_break(self) -> BreakTimer<'a> {
        BreakTimer::start_timer(self, self.worktime, self.breaktime)
    }
}

pub struct BreakTimer<'a> {
    worktime: &'a mut f64,
    breaktime: &'a mut f64,


}

impl<'a> BreakTimer<'a> {
    pub fn start_timer(&mut self, worktime: &'a mut f64, breaktime: &'a mut f64) {

        self.worktime = worktime;
        self.breaktime = breaktime;
        let elapsed = SystemTime::now();

        while &mut elapsed.elapsed().unwrap().as_secs_f64() < self.breaktime {
            println!("Work time left {:?}", *self.breaktime -  &elapsed.elapsed().unwrap().as_secs_f64())
        }

    }
}












#[cfg(test)]

mod tests {
  





    
}