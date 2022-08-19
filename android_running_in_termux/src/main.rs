
use notification::mqtt_client::*;

fn main() {
    // connect to MQTT broker running in phone
    let stream = connect_to_mqtt_broker("127.0.0.1:3333").unwrap_or_else(|error| {
            panic!("{:?}", error);
        });
    // subscribe to otp requests from lap
    subscribe_to_topic(&stream, "phone/otp/request").unwrap_or_else(|error| {
            panic!("{:?}", error);
        });
    // wait for requests to be triggered from lap and process them
    wait_and_process_request_from_lap(&stream);
    
}