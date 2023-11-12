use std::time::{Duration, SystemTime};



trait Timekeeper {
    // Should be on separate thread keeping time.
    fn start_timer(&end_time: &Option<u64>){
        let start = SystemTime::now();
        match end_time {
            Some(end_time) => {while start.elapsed().unwrap().as_secs() < end_time {


                                     }},
            _ =>  {while start.elapsed().unwrap().as_secs() < 25*60 {  // 25 min work

            }},
        }

        }
        
}


pub struct Pomodoro{
    work_time: Option<u64>,
    break_time: Option<u64>,
    work: Option<bool>,
    time_left: Option<u64>
    
}

impl Pomodoro {
    pub fn new_timer(work_time: u64, break_time:u64, work: bool) -> Pomodoro {
        // Need to handle different units as input
        // Seconds/Minutes/Hours
        let mut new_timer = Pomodoro{
            work_time: None,
            break_time: None,
            work: None,
            time_left: None,
        };
        
        new_timer.populate_timer(&work_time, &break_time, &work);

        return new_timer
    }

    fn populate_timer(&mut self, &work_time: &u64, &break_time:&u64, &work: &bool) {
        self.new_break_time(&break_time);
        self.new_work_time(&work_time);
        self.start_at_work(&work);
        self.time_left = if work {self.work_time} else {self.break_time};
        
    }
    fn new_work_time(&mut self, &work_time:&u64) {
        self.work_time = Some(work_time);
    }

    fn new_break_time(&mut self, &break_time:&u64){
        self.break_time = Some(break_time);
    }

    fn start_at_work(&mut self, &work: &bool) {
        self.work=Some(work)
    }

    fn start_timer(&mut self){
     
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
        let new_timer = Pomodoro::new_timer(10, 5, true);
        
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