use serde::{Deserialize, Serialize};
use crate::Packet;

// Ping
#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
    t: String,
    y: String,
    q: String,
    a: PingPayload
}

impl Packet for Ping {
    fn new(id: String) -> Ping {
        let r = Ping {
            t: String::from("aa"),
            y: String::from("q"),
            q: String::from("ping"),
            a: PingPayload::new(id)
        };
        return r;
    }

    fn to_bencode(&self) -> Vec<u8> {
        serde_bencoded::to_vec(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingPayload {
    id: String
}

impl PingPayload {
    fn new(id: String) -> PingPayload {
        PingPayload { id:String::from(id) }
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::Ping;
    use crate::Packet;

    #[test]
    fn test_ping_request_fields() {
        let pr = Ping::new(String::from("simon"));
        assert_eq!(pr.t, "aa");
        assert_eq!(pr.y, "q");
        assert_eq!(pr.q, "ping");
        assert_eq!(pr.a.id, "simon");
    }

    #[test]
    fn test_ping_request_bencode() {
        let pr = Ping::new(String::from("simon"));
        let data: Vec<u8> = pr.to_bencode();
        let s: String = String::from(std::str::from_utf8(&data).unwrap());
        assert_eq!(s, "d1:ad2:id5:simone1:q4:ping1:t2:aa1:y1:qe");
    }
}
