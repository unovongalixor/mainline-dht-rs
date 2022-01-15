use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {
    pub t: String,
    pub y: String,
    pub q: String,
    pub a: PingRequestPayload
}

impl PingRequest {
    pub fn new(id: String) -> PingRequest {
        let r = PingRequest {
            t: String::from("b7"),
            y: String::from("q"),
            q: String::from("ping"),
            a: PingRequestPayload::new(id)
        };
        return r;
    }

    pub fn send_to(&self, socket: UdpSocket, addr: String) {
        let data: Vec<u8> = self.to_bencode();
        match socket.send_to(&data, addr.clone()) {
            Ok(n) => {
                if n != data.len() {
                    return
                }
            },
            Err(e) => {
                println!("{}", e);
                return
            }
        }
    }

    pub fn to_bencode(&self) -> Vec<u8> {
        serde_bencoded::to_vec(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequestPayload {
    pub id: String
}

impl PingRequestPayload {
    fn new(id: String) -> PingRequestPayload {
        PingRequestPayload { id:String::from(id) }
    }
}
