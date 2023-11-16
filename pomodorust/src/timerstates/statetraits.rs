

pub trait State {
    fn start_work(self: Box<Self>) -> Box<dyn State>;
    fn start_break(self: Box<Self>) -> Box<dyn State>;
    fn stop_and_reset(self: Box<Self>) -> Box<dyn State>;
    fn pause_timer(self: Box<Self>) -> Box<dyn State>;
    fn start_timer(self: Box<Self>) -> Box<dyn State>;
}
pub trait Timekeeper {
    // Should be on separate thread keeping time.
    fn start_timer(&self){
       
        }
        
}