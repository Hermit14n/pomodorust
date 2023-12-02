use std::fmt;
use std::sync::{Arc, Mutex};
pub mod breaktimer;
pub mod worktimer;
use breaktimer::BreakTimer;
use worktimer::WorkTimer;

#[derive(Clone)]
pub struct Timer {
    worktime: f64,
    breaktime: f64,
    status: Arc<Mutex<Status>>,
    state: Arc<Mutex<State>>,
    time_left: Arc<Mutex<f64>>,
}
impl Timer {
    pub fn new_timer(
        worktime: f64,
        breaktime: f64,
        status: Arc<Mutex<Status>>,
        state: Arc<Mutex<State>>,
        time_left: Arc<Mutex<f64>>,
    ) -> Timer {
        Timer {
            worktime,
            breaktime,
            status,
            state,
            time_left,
        }
    }
    pub fn start_break(self) -> Option<BreakTimer> {
        // Start break consumes WorkTimer and creates a BreakTimer
        *self.state.lock().unwrap() = State::Break;
        *self.time_left.lock().unwrap() = self.breaktime;
        BreakTimer::start_timer(
            self.worktime,
            self.breaktime,
            self.status,
            self.state,
            self.time_left,
        )
    }
    pub fn start_work(self) -> Option<WorkTimer> {
        // start work consumes BreakTimer and returns a Worktimer
        *self.state.lock().unwrap() = State::Work;
        *self.time_left.lock().unwrap() = self.worktime;
        WorkTimer::start_timer(
            self.worktime,
            self.breaktime,
            self.status,
            self.state,
            self.time_left,
        )
    }
}

#[derive(PartialEq, Clone)]
pub enum Status {
    Pause,
    Reset,
    Stop,
    Active,
}

#[derive(Debug)]
pub enum State {
    Work,
    Break,
    Stopped,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Work => write!(f, "Work"),
            State::Break => write!(f, "Break"),
            State::Stopped => write!(f, "Stopped"),
        }
    }
}

#[cfg(test)]

mod tests {}
