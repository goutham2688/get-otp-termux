
use get_otp::mqtt_client::*;
use get_otp::windows_bindings::*;

fn main() {
    let noti_data: String;
    // connect to mqtt
    match connect_to_mqtt_broker("192.168.1.3:3333") {
        Ok(stream) => {
            // subscribe to otp requests from lap
            subscribe_to_topic(&stream, "phone/otp/response").unwrap_or_else(|error| {
                panic!("{:?}", error);
            });
            // send mqtt request to phone
            publish_to_topic(&stream, "phone/otp/request", "gib OTP").unwrap();
            // wait for response
            noti_data = wait_and_process_request_from_phone(&stream).unwrap_or_else(|error| {
                panic!("{:?}", error);
            });
        },
        Err(error) => {
            noti_data = error.to_string();
        },
    }
    
    
    //  Windows Notification fn
    display_win_toast_notification(&noti_data).unwrap_or_else(|error| {
        panic!("{:?}", error);
    });
    
    // notification does get shown sometimes.. adding sleep keeps the code active for sometime 
    std::thread::sleep(std::time::Duration::from_millis(200));
}
