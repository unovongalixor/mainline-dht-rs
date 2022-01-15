use serde::{Deserialize, Serialize};

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
