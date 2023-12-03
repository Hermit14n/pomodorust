pub fn convert_to_min_sec(time: f64) -> (i64, i64) {
    let minutes = time as i64 / 60;
    let seconds = time % 60.0;

    (minutes, seconds as i64)
}

fn convert_time_to_string(minutes: i64, seconds: i64) -> String {
    let mut minutes_string: String = Default::default();
    let mut seconds_string: String = Default::default();
    handle_single_digits(minutes, seconds, &mut minutes_string, &mut seconds_string)
}

fn handle_single_digits(
    minutes: i64,
    seconds: i64,
    minutes_string: &mut String,
    seconds_string: &mut String,
) -> String {
    if minutes < 10 {
        minutes_string.push('0');
    }
    if seconds < 10 {
        seconds_string.push('0');
    }

    minutes_string.push_str(&minutes.to_string());
    seconds_string.push_str(&seconds.to_string());
    minutes_string.to_owned() + ":" + seconds_string
}

pub fn format_time(seconds: f64) -> String {
    let mins: i64;
    let secs: i64;
    (mins, secs) = convert_to_min_sec(seconds);
    convert_time_to_string(mins, secs)
}

#[cfg(test)]
mod test {
    use crate::seconds_to_minutes::{
        convert_time_to_string, convert_to_min_sec, format_time, handle_single_digits,
    };
    #[test]
    pub fn test_conversion_1() {
        let seconds = 1400.0;
        let mins: i64;
        let secs: i64;
        (mins, secs) = convert_to_min_sec(seconds);
        assert_eq!(mins, 23);
        assert_eq!(secs, 20);
    }

    #[test]
    pub fn test_single_digit_handling() {
        let mut minutes_string: String = Default::default();
        let mut seconds_string: String = Default::default();
        let handled_digits = handle_single_digits(2, 2, &mut minutes_string, &mut seconds_string);
        assert_eq!(handled_digits, "02:02");
    }
    #[test]
    pub fn test_to_string() {
        let string_time = convert_time_to_string(10, 01);
        assert_eq!(string_time, "10:01");
    }

    #[test]
    pub fn seconds_to_min_sec_string() {
        let seconds = 1000.0;
        let string = format_time(seconds);
        assert_eq!("16:40", string);
    }
}
