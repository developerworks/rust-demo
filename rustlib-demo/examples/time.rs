use chrono::TimeZone;
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use chrono_tz::UTC;
use chrono_tz::Asia;
use chrono_tz::Europe::London;


fn main() {
    test();
    println!("{}", tz_name_chrono());

    t1();

    // Asia
}

pub fn test() {
    let now: DateTime<Local> = Local::now();
    let timezone = now.timezone();

    println!("{:?}", timezone);
    println!("{}", now.to_rfc2822());
}

// pub fn timezone_name() -> String {
//     let tz_name = unsafe {
//         let tz = timezone(std::ptr::null_mut());
//         let tz_name = CStr::from_ptr(tz.tz_name).to_str().unwrap();

//         println!("{}", tz_name);
//         tz_name
//     }
// }

pub fn tz_name_chrono() -> String {
    let local_time = Local::now();
    let tz_name = format!("{}", local_time.format("%Z"));

    tz_name
}

pub fn t1() {
    let tz: Tz = "Antarctica/South_Pole".parse().unwrap();
    let dt = tz.ymd(2016, 10, 22).and_hms(12, 0, 0);
    let utc = dt.with_timezone(&UTC);
    
    println!("{}", utc);
    println!("{}", dt);
    println!("{}", utc.with_timezone(&tz));
}
