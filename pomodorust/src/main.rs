use std::time::SystemTime;
use pomodorust::Pomodoro;
use pomodorust::Timekeeper;


fn main() {

    let new_pom = &Pomodoro::new_timer(10, 5, true);
    new_pom.start_timer()
    

    // Want to accept args
        // work time             --wt <f32>
        // break time            --bt <f32>
        // progress bar yes/no   --pb <bool>
        // pretty view           --pv <bool>


}
