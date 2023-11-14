use std::time::{Duration, SystemTime};
use std::io::Write;
use std::{sync::mpsc, thread};

use timerstates::states::{StopState, StartState};
mod timerstates;






pub trait Timekeeper {
    // Should be on separate thread keeping time.
    fn start_timer(&self){
       
        }
        
}


pub struct Pomodoro<T> {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    state: Option<T>,
}

// What states do I want?


impl<'a, T> Timekeeper for &'a Pomodoro<T> {

    fn start_timer(&self){

        let start = SystemTime::now();

        match self.work_time {

            Some(endtime) => {while start.elapsed().unwrap().as_secs() < endtime
                                     {
                                  print!("\rTime left: {}", endtime - start.elapsed().unwrap().as_secs());
                                  let _ = std::io::stdout().flush();
                                     }
                                  },

            _ =>  {while start.elapsed().unwrap().as_secs() < 25*60 {  // 25 min work

            }},
        }

        }
}

impl<StartState> Pomodoro<StartState> {

    pub fn new_timer(work_time: u64, break_time:u64, work: bool) -> Pomodoro<StartState> {
        // Need to handle different units as input
        // Seconds/Minutes/Hours
        let mut new_timer = Pomodoro::<StartState> {
            work_time: None,
            break_time: None,
            time_left: None,
            state: StartState,

        };
        
        new_timer.populate_timer_inputs::<StartState>(&work_time, &break_time, &work);

        return new_timer
    }

    pub fn populate_timer_inputs<StartState>(&mut self, &work_time: &u64, &break_time:&u64, &work: &bool) {
        self.new_break_time(&break_time);
        self.new_work_time(&work_time);
        self.time_left = if work {self.work_time} else {self.break_time};
        self.state = Option::Some(StartState {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left,
        });
        
    }
    fn new_work_time(&mut self, &work_time:&u64) {
        self.work_time = Some(work_time);
    }

    fn new_break_time(&mut self, &break_time:&u64){
        self.break_time = Some(break_time);
    }




    // Do I want this here?
    fn execute(&mut self) {
        // start new thread and transfer timer to the thread via "start timer"
        let (tx, rx) = mpsc::channel::<Self>();

        thread::spawn(move || {
            tx.send(self);
        });

    }

  


        

    }


pub struct Progress_bar;


pub struct Notification;


pub struct TextVisualizer;




#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn new_work_time(){
        let new_timer = Pomodoro::new_timer(10, 5,);
        
        assert_eq!(new_timer.work_time.unwrap(), 10);
    }
    #[test]
    fn new_break_time(){
        let new_timer = Pomodoro::new_timer(10, 5, true);
        
        assert_eq!(new_timer.break_time.unwrap(), 5);
    }

    #[test]
    fn new_start_work(){
        let new_timer = Pomodoro::new_timer(10, 5, false);
        
        assert_eq!(new_timer.work.unwrap(), false);
    }



    
}