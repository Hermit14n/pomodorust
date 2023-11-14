trait State {
    fn start_work(self: Box<Self>) -> Box<dyn State>;
    fn start_break(self: Box<Self>) -> Box<dyn State>;
    fn stop_and_reset(self: Box<Self>) -> Box<dyn State>;
    fn pause_timer(self: Box<Self>) -> Box<dyn State>;
}


pub struct StartState {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    // Initialized states reference default, empty state StartState
}



pub struct StopState<T> {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    prev_state: Option<T>,

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

pub struct WorkState<T> {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    prev_state: Option<T>,
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

pub struct BreakState<T> {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    prev_state: Option<T>,

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

pub struct PauseState<T> {
    work_time: Option<u64>,
    break_time: Option<u64>,
    time_left: Option<u64>,
    prev_state: Option<T>,
    
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
