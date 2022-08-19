use std::io::Write;
use std::net::{TcpStream,SocketAddr};
use std::str;

use mqtt::control::variable_header::ConnectReturnCode;
use mqtt::packet::*;
use mqtt::{Decodable, Encodable, QualityOfService};
use mqtt::{TopicFilter,TopicName};

pub fn connect_to_mqtt_broker(server: &str) -> Result<TcpStream, &'static str> {
    // println!("Connecting to MQTT Broker");
    if let Ok(mut stream) = TcpStream::connect_timeout(&server.parse::<SocketAddr>().unwrap(),std::time::Duration::from_secs(5)) {
        let mut conn = ConnectPacket::new("Lap_Termux");
        conn.set_clean_session(true);
        let mut buf = Vec::new();
        conn.encode(&mut buf).unwrap();
        stream.write_all(&buf[..]).unwrap();

        let connack = ConnackPacket::decode(&mut stream).unwrap();
        // println!("MQTT CONNACK {:?}", connack.connect_return_code());

        if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
            println!(
                "Failed to connect to server, return code {:?}",
                connack.connect_return_code()
            );
            return Err("Failed to connect to MQTT broker");
        }
        return Ok(stream);
    } else {
        return Err("Unable to connect to broker");
    }
}

pub fn subscribe_to_topic(mut stream: &TcpStream, topic_name: &str) -> Result<bool, &'static str> {
    let topic_filter = TopicFilter::new(topic_name).unwrap();
    let channel_filters: Vec<(TopicFilter, QualityOfService)> = vec![(topic_filter,QualityOfService::Level0)];
    let sub = SubscribePacket::new(10, channel_filters);
    match sub.encode(&mut stream){
            Ok(..) => {
                // println!("subscriptions sent to broker");
            },
            Err(..) => {
                println!("error subscribing to topic");
                return Err("ERROR: subscribing to topic");
            },
        }
    let _suback = SubackPacket::decode(&mut stream).unwrap();
    // println!("MQTT SUBACK {:?}", suback);
    // println!("subcribed succeffully to topic");

    Ok(true)
}

pub fn publish_to_topic(mut stream: &TcpStream, topic_name: &str, payload: &str) -> Result<bool, &'static str> {
    let response_topic_name = TopicName::new(topic_name).unwrap();
    let mqtt_res = PublishPacket::new(response_topic_name,QoSWithPacketIdentifier::Level0,payload);
    mqtt_res.encode(&mut stream).unwrap();
    Ok(true)
}

pub fn wait_and_process_request_from_phone(mut stream: &TcpStream) -> Result<String, String> {
    loop {
        let packet = match VariablePacket::decode(&mut stream) {
            Ok(pk) => pk,
            Err(err) => {
                println!("Error in receiving packet {:?}", err);
                continue;
            }
        };
        // println!("PACKET {:?}", packet);

        match packet {
            VariablePacket::PingreqPacket(..) => {
                let pingresp = PingrespPacket::new();
                println!("Sending Ping response {:?}", pingresp);
                pingresp.encode(&mut stream).unwrap();
            },
            VariablePacket::DisconnectPacket(..) => {
                return Err("Error: disconnect".to_string());
            },
            VariablePacket::PublishPacket(data) => {
                // println!("{:?}",data.topic_name());
                // mosquitto_sub –d –t phone/otp/response
                // mosquitto_pub -d -t phone/otp/request -m "OTP gib"
                match data.topic_name() {
                    "phone/otp/response" => {
                            let msg = match str::from_utf8(data.payload()) {
                                Ok(msg) => msg,
                                Err(err) => {
                                    println!("Failed to decode publish message {:?}", err);
                                    return Err("Error: decoding payload".to_string());
                                }
                            };
                            // println!("OTP: received: {}",msg);
                            println!("{}",msg);
                            return Ok(msg.to_string());
                        },

                    _ => {
                            println!("ERROR: unknown topic");
                        }
                }
            },

            _ => {
                // Ignore other packets in pub client
            }
        }
    }
} 