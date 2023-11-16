use crate::timerstates::{breakstate::BreakState, pausestate::PauseState, stopstate::StopState};
use crate::timerstates::statetraits::{State, Timekeeper};

pub struct WorkState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,
    // Possibly add timer functionality here
}


impl<T> State for WorkState<T> {
    fn start_break(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(BreakState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }

    fn start_timer(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(PauseState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }

    fn pause_timer(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(PauseState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }

    fn stop_and_reset(self: Box<Self>) -> Box<dyn State + 'static> {
        Box::new(StopState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(*self),
        })
    }

    fn start_work(self: Box<Self>) -> Box<dyn State + 'static> {
        println!("Already working");
        self

    }

}
