use chrono::{FixedOffset, TimeZone, Utc};

pub struct PreformattedDateTime {
    pub human_readable: String,
    pub machine_friendly: String,
}

impl PreformattedDateTime {
    pub fn from_timestamp(timestamp: i64) -> PreformattedDateTime {
        let tz = FixedOffset::east_opt(8 * 3600).unwrap();
        let utc_time = Utc.timestamp_opt(timestamp, 0).unwrap();
        let tz_time = utc_time.with_timezone(&tz);
        let human_readable = format!("{}", tz_time.format("%Y-%m-%d"));
        let machine_friendly = tz_time.to_rfc3339();
        PreformattedDateTime {
            human_readable,
            machine_friendly,
        }
    }
}
