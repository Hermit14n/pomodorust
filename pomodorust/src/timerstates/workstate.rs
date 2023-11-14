use crate::timerstates::{breakstate::BreakState, pausestate::PauseState, stopstate::StopState};
use crate::timerstates::statetraits;

pub struct WorkState<T> {
    pub work_time: Option<u64>,
    pub break_time: Option<u64>,
    pub time_left: Option<u64>,
    pub prev_state: Option<T>,
    // Possibly add timer functionality here
}

impl<T> WorkState<T> {
    pub fn start_break(self) -> BreakState<Self> {
        BreakState::<Self> {
            work_time: self.work_time,
            break_time: self.break_time,
            time_left: self.time_left, 
            prev_state: Option::Some(self),
        }
    }

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

}
