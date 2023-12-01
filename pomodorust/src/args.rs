use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TimerArgs {
    /// Work time in minutes
    #[arg(short = 'w', long, default_value_t = 25.0)]
    pub worktime: f64,

    /// Break time in minutes
    #[arg(short = 'b', long, default_value_t = 5.0)]
    pub breaktime: f64,

    /// Number of work, break rounds, default is infinite
    #[arg(short = 'r', long, default_value_t = -1)]
    pub rounds: i32,
}
