use crate::timerstates::{pausestate::PauseState, workstate::WorkState, stopstate::StopState};
use crate::timerstates::statetraits;

pub struct BreakState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,

}

impl<T> BreakState<T> {

    pub fn pause_timer(self) -> PauseState<Self> {
        PauseState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(self),
        }
    }

    pub fn stop_and_reset(self) -> StopState<Self> {
        StopState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left,  
            prev_state: Option::Some(self),
    }
}

    pub fn start_work(self) -> WorkState<Self> {
        WorkState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left,
            prev_state: Option::Some(self), 
        }
    }
}