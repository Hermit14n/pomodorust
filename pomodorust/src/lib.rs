use std::fmt;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
        *self.time_left.lock().unwrap() = self.breaktime.clone();
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
        *self.time_left.lock().unwrap() = self.worktime.clone();
        WorkTimer::start_timer(
            self.worktime,
            self.breaktime,
            self.status,
            self.state,
            self.time_left,
        )
    }
}

#[derive(Clone)]
pub struct WorkTimer {
    worktime: f64,
    breaktime: f64,
    status: Arc<Mutex<Status>>,
    state: Arc<Mutex<State>>,
    time_left: Arc<Mutex<f64>>,
}

impl WorkTimer {
    pub fn start_timer(
        worktime: f64,
        breaktime: f64,
        status: Arc<Mutex<Status>>,
        state: Arc<Mutex<State>>,
        time_left: Arc<Mutex<f64>>,
    ) -> Option<WorkTimer> {
        let timer = WorkTimer {
            worktime,
            breaktime,
            status,
            state,
            time_left,
        };
        let mut elapsed = Instant::now();
        let mut pause_elapsed = 0.0;
        while *timer.time_left.lock().unwrap() > 0.0 {
            // TODO do this loop based on worktime_left > 0, use timers to subtract from worktime left
            if *timer.status.lock().unwrap() == Status::Active {
                stdout().flush().unwrap();
                *timer.time_left.lock().unwrap() =
                    timer.worktime - elapsed.elapsed().as_secs_f64() - pause_elapsed;
            } else if *timer.status.lock().unwrap() == Status::Pause {
                pause_elapsed += elapsed.elapsed().as_secs_f64();
                loop {
                    stdout().flush().unwrap();
                    // failure to pause printed time problem is here
                    if *timer.status.lock().unwrap() == Status::Active {
                        elapsed = Instant::now();
                        break;
                    }
                }
            }
        }

        Some(timer)
    }

    pub fn start_break(self) -> Option<BreakTimer> {
        // Start break consumes WorkTimer and creates a BreakTimer
        *self.state.lock().unwrap() = State::Break;
        *self.time_left.lock().unwrap() = self.breaktime.clone();
        BreakTimer::start_timer(
            self.worktime,
            self.breaktime,
            self.status,
            self.state,
            self.time_left,
        )
    }
}

#[derive(Clone)]
pub struct BreakTimer {
    worktime: f64,
    breaktime: f64,
    status: Arc<Mutex<Status>>,
    state: Arc<Mutex<State>>,
    time_left: Arc<Mutex<f64>>,
}

impl BreakTimer {
    pub fn start_timer(
        worktime: f64,
        breaktime: f64,
        status: Arc<Mutex<Status>>,
        state: Arc<Mutex<State>>,
        time_left: Arc<Mutex<f64>>,
    ) -> Option<BreakTimer> {
        let timer = BreakTimer {
            worktime,
            breaktime,
            status,
            state,
            time_left,
        };
        let mut elapsed = Instant::now();
        let mut pause_elapsed = 0.0;

        while *timer.time_left.lock().unwrap() > 0.0 {
            // TODO do this loop based on worktime_left > 0, use timers to subtract from worktime left
            if *timer.status.lock().unwrap() == Status::Active {
                //stdout().flush().unwrap();
                *timer.time_left.lock().unwrap() =
                    timer.breaktime - elapsed.elapsed().as_secs_f64() - pause_elapsed;
            } else if *timer.status.lock().unwrap() == Status::Pause {
                pause_elapsed += elapsed.elapsed().as_secs_f64();
                loop {
                    stdout().flush().unwrap();

                    // failure to pause printed time problem is here
                    if *timer.status.lock().unwrap() == Status::Active {
                        elapsed = Instant::now();
                        break;
                    }
                }
            }
        }

        Some(timer)
    }
    pub fn start_work(self) -> Option<WorkTimer> {
        *self.state.lock().unwrap() = State::Work;
        *self.time_left.lock().unwrap() = self.worktime.clone();
        // start work consumes BreakTimer and returns a Worktimer
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
