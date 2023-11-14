use crate::timerstates::{stopstate::StopState, workstate::WorkState};
use crate::timerstates::statetraits;

pub struct PauseState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,
    
}

impl<T> PauseState<T> {

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