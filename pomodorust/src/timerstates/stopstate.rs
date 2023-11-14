use crate::timerstates::{workstate::WorkState, breakstate::BreakState};
use crate::timerstates::statetraits;

pub struct StopState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,

}

impl<T> StopState<T> {  // have to specify type at impl, so rust knows its a generic type and not specific
    pub fn start_work(self) -> WorkState<Self> {
        WorkState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(self),
        }
    }

    pub fn start_break(self) -> BreakState<Self> {
        BreakState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(self),
        }
    }
}