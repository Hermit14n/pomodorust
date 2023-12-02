use crate::{worktimer::WorkTimer, State, Status};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
            if *timer.status.lock().unwrap() == Status::Active {
                *timer.time_left.lock().unwrap() =
                    timer.breaktime - elapsed.elapsed().as_secs_f64() - pause_elapsed;
            } else if *timer.status.lock().unwrap() == Status::Pause {
                pause_elapsed += elapsed.elapsed().as_secs_f64();
                loop {
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
        *self.time_left.lock().unwrap() = self.worktime;
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
