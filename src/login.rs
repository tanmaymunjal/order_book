use chrono::{DateTime, Utc};
use std::rc::Rc;

pub fn login(
    sender_id: String,
    username: String,
    password: String,
    receiving_firm: String,
    receiving_target: String,
) -> String {
    let current_time: String = get_current_datetime();
    let login:Rc<String> = Rc::new(format!("8=FIX.4.4|9=114| 35=A |34=1| 49={sender_id}| 52={current_time}| 56={receiving_firm} |57={receiving_target}| 553={username}| 554={password}| 98=0 |108=30 |141=Y|"));
    let new_login: String = login.as_ref().to_owned() + " 10=" + &calculate_checksum(&login) + "|";
    new_login
}

pub fn get_current_datetime() -> String {
    let current_time: DateTime<Utc> = Utc::now();
    format_time(current_time)
}

pub fn format_time(current_time: DateTime<Utc>) -> String {
    // Parse the input string into a DateTime object
    let parsed_dt = DateTime::parse_from_rfc3339(&current_time.to_string()).unwrap();

    // Convert the parsed DateTime to the desired format
    let formatted_dt = parsed_dt.format("%Y%m%d-%H:%M:%S%.3f").to_string();
    formatted_dt
}

fn calculate_checksum(message: &str) -> String {
    let bytes = message.as_bytes();
    let mut checksum: u16 = 0;

    for &byte in bytes {
        checksum = (checksum + byte as u16) % 256;
    }

    format!("{:03}", checksum)
}
