pub use chrono::{Datelike, NaiveDate, Weekday};
pub use chrono::Utc;
pub use chrono::Weekday as wd;
pub use chrono::offset::TimeZone;
pub fn middle_day(p0: i32) -> Option<Weekday> {

    if p0 < 1 || p0 % 4 == 0 && p0% 100 != 0 || p0% 100 == 0 && p0 % 400 != 0 {
        return None;
    }
    return Option::from(
        NaiveDate::parse_from_str(format!("{}-07-02", p0).as_str(), "%Y-%m-%d")
            .unwrap()
            .weekday());
}
