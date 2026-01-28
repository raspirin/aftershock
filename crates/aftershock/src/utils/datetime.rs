#[cfg(not(target_arch = "wasm32"))]
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

// Type alias that selects implementation based on target architecture
// Use UserLocalDateTime (js_sys) in wasm32 to avoid chrono dependency
// Use PreformattedDateTime (chrono) in SSR/other targets
#[cfg(target_arch = "wasm32")]
pub type AppDateTime = UserLocalDateTime;

#[cfg(not(target_arch = "wasm32"))]
pub type AppDateTime = PreformattedDateTime;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct PreformattedDateTime {
    year: i32,
    month: u32,
    day: u32,
    // pub orig: i64,
    human_readable: String,
    machine_friendly: String,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub struct UserLocalDateTime {
    year: i32,
    month: u32,
    day: u32,
    human_readable: String,
    machine_friendly: String,
}

#[cfg(target_arch = "wasm32")]
impl UserLocalDateTime {
    // Note: new() constructor removed as it's redundant with from_timestamp()
}

#[cfg(target_arch = "wasm32")]
impl DateTime for UserLocalDateTime {
    fn from_timestamp(timestamp: i64) -> UserLocalDateTime {
        // Use browser's Date API via js_sys
        // Create date from timestamp (milliseconds)
        let timestamp_ms = (timestamp * 1000) as f64; // Convert seconds to milliseconds
        let js_date = js_sys::Date::new(&timestamp_ms.into());
        
        let year = js_date.get_full_year() as i32;
        // JavaScript getMonth() returns 0-11, so we add 1
        let month = (js_date.get_month() + 1) as u32;
        let day = js_date.get_date() as u32;
        
        // Format human readable: YYYY-MM-DD
        let human_readable = format!(
            "{:04}-{:02}-{:02}",
            year,
            month,
            day
        );
        
        // Format machine friendly: ISO 8601 string
        let machine_friendly = js_date.to_iso_string().as_string().unwrap_or_default();
        
        UserLocalDateTime {
            year,
            month,
            day,
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
