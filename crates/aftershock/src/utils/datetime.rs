use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};

#[derive(Clone)]
pub struct DateTriplet {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone)]
pub struct CommonTime<C: UnixTimestampConverter> {
    converter: C,
    pub date_triplet: DateTriplet,
    pub human_readable: String,
    pub machine_friendly: String,
}

impl<C: UnixTimestampConverter> CommonTime<C> {
    pub fn from_timestamp(timestamp: i64) -> Self {
        let converter = C::from_timestamp(timestamp);
        let date_triplet = converter.datetime();
        let human_readable = converter.render_pretty();
        let machine_friendly = converter.render_rfc3339();
        Self {
            converter,
            date_triplet,
            human_readable,
            machine_friendly,
        }
    }
}

impl DateTriplet {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn month_symbol(&self) -> Option<&'static str> {
        match self.month {
            1 => Some("Jan"),
            2 => Some("Feb"),
            3 => Some("Mar"),
            4 => Some("Apr"),
            5 => Some("May"),
            6 => Some("Jun"),
            7 => Some("Jul"),
            8 => Some("Aug"),
            9 => Some("Sep"),
            10 => Some("Oct"),
            11 => Some("Nov"),
            12 => Some("Dec"),
            _ => None,
        }
    }
}

pub trait UnixTimestampConverter {
    const FMT: &'static str = "%Y-%m-%d";

    fn from_timestamp(timestamp: i64) -> Self;
    fn datetime(&self) -> DateTriplet;
    fn render_rfc3339(&self) -> String;
    fn render_pretty(&self) -> String;
}

pub struct AutoFormattedDateTime {
    pub orig: i64,
}

#[derive(Clone, Debug)]
pub struct StaticFormattedDateTime {
    pub date: DateTime<FixedOffset>,
    // pub orig: i64,
}

impl UnixTimestampConverter for StaticFormattedDateTime {
    fn datetime(&self) -> DateTriplet {
        DateTriplet {
            year: self.date.year(),
            month: self.date.month(),
            day: self.date.day(),
        }
    }

    fn render_rfc3339(&self) -> String {
        self.date.to_rfc3339()
    }

    fn render_pretty(&self) -> String {
        format!("{}", self.date.format(Self::FMT))
    }

    fn from_timestamp(timestamp: i64) -> Self {
        // let utc = Utc.timestamp_opt(timestamp, 0).unwrap();
        // let local = Local.from_utc_datetime(&utc.naive_utc());
        let tz = FixedOffset::east_opt(8 * 3600).unwrap();
        let utc_time = Utc.timestamp_opt(timestamp, 0).unwrap();
        let tz_time = utc_time.with_timezone(&tz);
        Self {
            date: tz_time,
            // orig: timestamp,
        }
    }
}

// impl StaticFormattedDateTime {
//     // FIXME: no unwrap here
//     fn from_timestamp(timestamp: i64) -> Self {
//         let tz = FixedOffset::east_opt(8 * 3600).unwrap();
//         let utc_time = Utc.timestamp_opt(timestamp, 0).unwrap();
//         let tz_time = utc_time.with_timezone(&tz);
//         let human_readable = format!("{}", tz_time.format("%Y-%m-%d"));
//         let machine_friendly = tz_time.to_rfc3339();
//         let year = tz_time.year();
//         let month = tz_time.month();
//         let day = tz_time.day();
//         Self { orig: timestamp }
//     }
// }
