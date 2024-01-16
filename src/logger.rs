use crate::chalk::{self, ForegroundType};
use chrono::{Local, Timelike};

pub fn log(message: &str) -> () {
    fn time() -> String {
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let am_pm = if hour < 12 { "AM" } else { "PM" };
        let hour_12 = if hour > 12 { hour - 12 } else { hour };

        format!("{:02}:{:02} {}", hour_12, minute, am_pm)
    }

    /*
    fn time_seconds() -> String {
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let am_pm = if hour < 12 { "AM" } else { "PM" };
        let hour_12 = if hour > 12 { hour - 12 } else { hour };

        format!("{:02}:{:02}:{:02} {}", hour_12, minute, second, am_pm)
    }
    */

    println!("[{0}] {1} {2}", time().as_str(), ">", message);
}
pub fn info(message: &str) -> () {
    let value: &str = &chalk::Foreground::red("[📄 Info!]");
    println!("{0} {1} {2}", value, ">", message)
}

pub fn new() -> () {}
