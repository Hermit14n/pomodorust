pub fn convert_to_min_sec(time: f64) -> (i64, i64) {

    let minutes = time as i64 /60;
    let seconds = time % 60.0;

    (minutes, seconds as i64)
}










#[cfg(test)]
mod test{
    use crate::seconds_to_minutes::convert_to_min_sec;
    #[test]
    pub fn test_conversion_1(){
        let seconds = 1400.0;
        let mins: i64;
        let secs: i64;
        (mins, secs) = convert_to_min_sec(seconds);
        assert_eq!(mins, 23);
        assert_eq!(secs, 20);
    }



}