use crate::timerstates::statetraits::{Timekeeper, State};
use crate::timerstates::{pausestate::PauseState, workstate::WorkState, stopstate::StopState, breakstate::BreakState};
pub struct StartState {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    // Initialized states reference default, empty state StartState
}

impl State for StartState {
    fn start_work(self: Box<Self>) -> Box<dyn State + 'static>{
        Box::new(WorkState {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.work_time,
            prev_state: Option::Some(*self),
    })
    }
    fn start_break(self: Box<Self>) -> Box<dyn State + 'static> {

    }
    fn stop_and_reset(self: Box<Self>) -> Box<dyn State + 'static> {

    }
    fn pause_timer(self: Box<Self>) -> Box<dyn State + 'static> {

    }
}








