use chrono::{Datelike, FixedOffset, TimeZone, Utc};

#[derive(Clone, Debug)]
pub struct PreformattedDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    // pub orig: i64,
    pub human_readable: String,
    pub machine_friendly: String,
}

impl PreformattedDateTime {
    // FIXME: no unwrap here
    pub fn from_timestamp(timestamp: i64) -> PreformattedDateTime {
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

    pub fn month_to_abbr(&self) -> &'static str {
        match self.month {
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
