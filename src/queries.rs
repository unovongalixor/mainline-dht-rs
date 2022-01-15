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
            t: String::from("aa"),
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

#[cfg(test)]
mod tests {
    use crate::queries::PingRequest;

    #[test]
    fn test_ping_request_fields() {
        let pr = PingRequest::new(String::from("simon"));
        assert_eq!(pr.t, "aa");
        assert_eq!(pr.y, "q");
        assert_eq!(pr.q, "ping");
        assert_eq!(pr.a.id, "simon");
    }

    #[test]
    fn test_ping_request_bencode() {
        let pr = PingRequest::new(String::from("simon"));
        let data: Vec<u8> = pr.to_bencode();
        let s: String = String::from(std::str::from_utf8(&data).unwrap());
        assert_eq!(s, "d1:ad2:id5:simone1:q4:ping1:t2:aa1:y1:qe");
    }
}
