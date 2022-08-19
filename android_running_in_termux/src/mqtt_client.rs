
use crate::parse_for_otp_in_notifications::get_otp;

use std::io::Write;
use std::net::TcpStream;

use mqtt::control::variable_header::ConnectReturnCode;
use mqtt::packet::*;
use mqtt::{Decodable, Encodable, QualityOfService};
use mqtt::{TopicFilter,TopicName};

pub fn connect_to_mqtt_broker(server: &str) -> Result<TcpStream, &'static str> {
    println!("Connecting to MQTT Broker");
    if let Ok(mut stream) = TcpStream::connect(server) {
        let mut conn = ConnectPacket::new("Phone_Termux");
        conn.set_clean_session(true);
        let mut buf = Vec::new();
        conn.encode(&mut buf).unwrap();
        stream.write_all(&buf[..]).unwrap();

        let connack = ConnackPacket::decode(&mut stream).unwrap();
        println!("MQTT CONNACK {:?}", connack.connect_return_code());

        if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
            println!(
                "Failed to connect to server, return code {:?}",
                connack.connect_return_code()
            );
            return Err("Failed to connect to MQTT broker");
        }
        return Ok(stream);
    } else {
        return Err("TcpStream connect error");
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
    println!("subcribed succeffully to topic");

    Ok(true)
}

pub fn wait_and_process_request_from_lap(mut stream: &TcpStream) {
    loop {
        let packet = match VariablePacket::decode(&mut stream) {
            Ok(pk) => pk,
            Err(err) => {
                println!("Error in receiving packet {:?}", err);
                continue;
            }
        };
        println!("PACKET {:?}", packet);

        match packet {
            VariablePacket::PingreqPacket(..) => {
                let pingresp = PingrespPacket::new();
                println!("Sending Ping response {:?}", pingresp);
                pingresp.encode(&mut stream).unwrap();
            },
            VariablePacket::DisconnectPacket(..) => {
                break;
            },
            VariablePacket::PublishPacket(data) => {
                println!("{:?}",data.topic_name());
                // mosquitto_sub –d –t phone/otp/response
                // mosquitto_pub -d -t phone/otp/request -m "OTP gib"
                match data.topic_name() {
                    "phone/otp/request" => {
                            let response_string: String;
                            match get_otp() {
                                    Ok(otp_string) => {
                                        println!("OTP String found: {}",otp_string);
                                        response_string = otp_string;
                                    },
                                    Err(error) => {
                                        println!("ERROR locating OTP in notifications {}",error);
                                        response_string = String::from("ERROR");
                                    }
                                }
                            let response_topic_name = TopicName::new("phone/otp/response").unwrap();
                            let mqtt_res = PublishPacket::new(response_topic_name,QoSWithPacketIdentifier::Level0,response_string);
                            mqtt_res.encode(&mut stream).unwrap();
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