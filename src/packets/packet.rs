use std::net::UdpSocket;

/// Packet defines common behavior for all message packets
/// like functions for bencoding data and sending a packet via a UdpSocket
pub trait Packet {
    fn new(id: String) -> Self; 
    fn to_bencode(&self) -> Vec<u8>;    

    fn send_to(&self, socket: UdpSocket, addr: String) {
        let data: Vec<u8> = self.to_bencode();
        println!("{}", std::str::from_utf8(&data).unwrap());
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
}

