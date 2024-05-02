extern crate chrono;

use chrono::prelude::*;

pub(crate) fn generate_message() -> String{
    let today = get_today();
    format!("Heute ist der {}", today)
}

fn get_today() -> String{
    let local_time = Local::now();
    local_time.format("%Y-%m-%d").to_string()
}