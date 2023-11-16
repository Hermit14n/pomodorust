use crate::timerstates::{workstate::WorkState, breakstate::BreakState};
use crate::timerstates::statetraits::{State, Timekeeper}
;
pub struct StopState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,

}

impl<T> State for StopState<T> {  // have to specify type at impl, so rust knows its a generic type and not specific
    fn start_work(self: Box<Self>) -> Box<dyn State + 'static>  {
        Box::new(WorkState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }
    fn start_timer(self: Box<Self>) -> Box<dyn State + 'static> {
        println!("Stopped, choose break or work");
        self

    }

    fn start_break(self: Box<Self>) -> Box<dyn State + 'static>  {
        Box::new(BreakState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }

    fn stop_and_reset(self: Box<Self>) -> Box<dyn State + 'static>{
        println!("Already stopped!");
        self
    }

    fn pause_timer(self: Box<Self>) -> Box<dyn State + 'static>{
        println!("Stopped, nothing to pause!");
        self
    }
}