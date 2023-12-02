use crate::{breaktimer::BreakTimer, State, Status};
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
            timer.adapt_timer_to_state(&mut elapsed, &mut pause_elapsed);
        }

        Some(timer)
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

    pub fn adapt_timer_to_state(&self, elapsed: &mut Instant, pause_elapsed: &mut f64) {
        if *self.status.lock().unwrap() == Status::Active {
            *self.time_left.lock().unwrap() =
                self.breaktime - elapsed.elapsed().as_secs_f64() - *pause_elapsed;
        } else if *self.status.lock().unwrap() == Status::Pause {
            *pause_elapsed += elapsed.elapsed().as_secs_f64();
            loop {
                if *self.status.lock().unwrap() == Status::Active {
                    *elapsed = Instant::now();
                    break;
                }
            }
        }
    }
}
