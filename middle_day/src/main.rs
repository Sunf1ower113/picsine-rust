use chrono::TimeZone;
use middle_day::*;

fn main() {
    // let date = Utc.with_ymd_and_hms(2011, 12, 2, 21, 12, 09);

    println!("{:?}", middle_day(2022).unwrap());
}

