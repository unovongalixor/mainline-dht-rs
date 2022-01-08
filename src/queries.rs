use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {
    pub t: String,
    pub y: String,
    pub q: String,
    pub a: PingRequestPayload
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequestPayload {
    pub id: String
}
