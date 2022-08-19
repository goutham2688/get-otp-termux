
use std::convert::TryInto;
use std::time::{Duration, SystemTime};
use std::process::Command;
use serde::{Deserialize, Serialize};
use regex::Regex;

// 15 minutes converted to seconds
const NOTIFICATION_DEALY_TIME_IN_SEC: u64 = 15 * 60 ; 
const MILLISECONDS_IN_A_SECOND:usize = 1000;
#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    id: isize,
    tag: String,
    key: String,
    group: String,
    #[serde(rename="packageName")]
    package_name: String,
    title: String,
    content: String,
    when: usize,
}

pub fn get_otp() -> Result<String, &'static str> {
    let all_notifications = get_all_notifications()?;
    let otp_str = parse_json_for_otp(all_notifications)?;
    Ok(otp_str)
}

fn get_all_notifications() -> Result<String, &'static str> {
    // run the termux command to get all the notifications from the phone
    let output = Command::new("termux-notification-list")
        .output()
        .expect("ls command failed to start");

    // println!("status: {}", output.status);
    // if the returned output was success process the string, else propogate Err
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("ERROR: running termux termux-notification-list")
    }
}

fn check_message_validity(msg: &str,time: usize) -> bool {
    // remove milliseconds from unix timestamp of notification
    let adjusted_time:usize = time/MILLISECONDS_IN_A_SECOND;
    // check if message has any words like code or otp (?i for non case sensitive search)
    let side_re = Regex::new(r"((?i)otp|code)").unwrap();
    let word_check:bool = side_re.is_match(msg);
    // get unix time for 15 mins before current time
    let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time_15 = time_now.checked_sub(Duration::from_secs(NOTIFICATION_DEALY_TIME_IN_SEC)).unwrap(); // 15 minutes
    let mut time_check: bool = false;

    println!("time_now: {:#?}  time_15: {:#?}  msg time: {:#?}",time_now,time_15,adjusted_time);
    // ignore msg if the notification was before 15 minutes
    if adjusted_time > time_15.as_secs().try_into().unwrap(){
        println!("msg within time");
        time_check = true;
    } else {
        println!("msg outside time");
    }
    return word_check && time_check;
}

fn parse_json_for_otp(all_notifications: String) -> Result<String, &'static str>  {
    // deserialize json data
    let notis: Vec<Notification> = match serde_json::from_str(&all_notifications) {
        Ok(data) => data,
        Err(error) => {
            println!("serde_json error parse info {:#?}",error);
            return Err("ERROR: parsing json data ");
        },
    };
    let otp_re = Regex::new(r"(\d{6,})").unwrap();
    // loop through all notifications
    for noti in notis {
        // check for notification from SMS app "Signal" here
        if noti.package_name == "org.thoughtcrime.securesms" {
            println!("content: {}",noti.content);
            // check if SMS has no greater than 6 digits, 
            if otp_re.is_match(&noti.content) && check_message_validity(&noti.content, noti.when) {
                let otp_captures =  otp_re.captures(&noti.content).unwrap();
                // OTP would be captured in the 0th index
                return Ok(otp_captures.get(0).unwrap().as_str().to_string());
            } else {
                // Do Nothing here
            }
        } else {
            // Do Nothing here
        }
    }
    Err("ERROR: no OTP found")
}