use chrono::{Datelike, FixedOffset, TimeZone, Utc};

pub trait DateTime {
    fn from_timestamp(timestamp: i64) -> Self;

    fn year(&self) -> i32;
    fn month(&self) -> u32;
    fn day(&self) -> u32;

    fn human_readable(&self) -> &str;
    fn machine_friendly(&self) -> &str;

    fn month_to_abbr(&self) -> &'static str {
        match self.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreformattedDateTime {
    year: i32,
    month: u32,
    day: u32,
    // pub orig: i64,
    human_readable: String,
    machine_friendly: String,
}

impl DateTime for PreformattedDateTime {
    // FIXME: no unwrap here
    fn from_timestamp(timestamp: i64) -> PreformattedDateTime {
        let tz = FixedOffset::east_opt(8 * 3600).unwrap();
        let utc_time = Utc.timestamp_opt(timestamp, 0).unwrap();
        let tz_time = utc_time.with_timezone(&tz);
        let human_readable = format!("{}", tz_time.format("%Y-%m-%d"));
        let machine_friendly = tz_time.to_rfc3339();
        let year = tz_time.year();
        let month = tz_time.month();
        let day = tz_time.day();
        PreformattedDateTime {
            year,
            month,
            day,
            // orig: timestamp,
            human_readable,
            machine_friendly,
        }
    }

    fn year(&self) -> i32 {
        self.year
    }
    
    fn month(&self) -> u32 {
        self.month
    }
    
    fn day(&self) -> u32 {
        self.day
    }
    
    fn human_readable(&self) -> &str {
        &self.human_readable
    }
    
    fn machine_friendly(&self) -> &str {
        &self.machine_friendly
    }
}
