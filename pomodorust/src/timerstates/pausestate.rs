use crate::timerstates::{stopstate::StopState, workstate::WorkState};
use crate::timerstates::statetraits::{State, Timekeeper};

pub struct PauseState<T: State> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,
    
}

impl<T: State> State for PauseState<T: State> {

     fn stop_and_reset(self: Box<Self>) -> Box<dyn State + 'static> {    
            Box::new(StopState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left,
            prev_state: Option::Some(*self),  
    })

    
}

fn start_timer(self: Box<Self>) -> Box<dyn State + 'static> {
        self.prev_state.take()
    
}
     fn start_work(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(WorkState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left,
            prev_state: Option::Some(*self), 
        })
    }

     fn start_break(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(PauseState::<Self> { 
            work_time: self.work_time, 
            break_time: self.break_time, 
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
         })
    }

    fn pause_timer(self: Box<Self>) ->  Box<dyn State + 'static>{
        println!("Already paused!");
        self
    }
}