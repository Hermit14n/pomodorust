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

#[cfg(test)]
mod tests {
    use clap::Parser;
    use crate::TimerArgs;
    #[test]
    pub fn test_defaults(){
        let args = TimerArgs::parse_from(vec!["test"]);
        assert_eq!(args.worktime, 25.0);
        assert_eq!(args.breaktime, 5.0);
        assert_eq!(args.rounds, -1);
    }
    #[test]
    pub fn test_inputs(){
        let args = TimerArgs::parse_from(vec!["test", "-w", "30.0", "-b", "30.0", "-r", "10"]);
        assert_eq!(args.worktime, 30.0);
        assert_eq!(args.breaktime, 30.0);
        assert_eq!(args.rounds, 10);
    }
}
