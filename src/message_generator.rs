extern crate chrono;

use chrono::prelude::*;

pub fn generate_message() -> String{
    let today = get_today();
    let message = format!("Version: {}", today);
    message
}

fn get_today() -> String{
    let local_time = Local::now();
    local_time.format("%Y-%m-%d").to_string()
}