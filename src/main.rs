pub mod request_packets;

#[macro_use] extern crate clap;

use clap::App;
use ansi_term::Colour;
use ansi_term::Style;
use request_packets::ping_request::PingRequest;
use std::net::UdpSocket;
use std::time::Duration;
use request_packets::request_packet::RequestPacket;


fn main() {
    println!();
    println!("{}", Colour::Green.paint("Welcome to the Mainline DHT Crawler"));
    println!();
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let magnet_uri = matches.value_of("magnet_uri").unwrap();
    println!("Crawling the DHT for torrent {}", Style::new().bold().paint(magnet_uri));

    println!("pinging bootstrap nodes..");
    let bootstrap_servers = vec![
        String::from("router.bittorrent.com:6881")
    ];

    let ping_request = PingRequest::new(String::from("abcdefghij0123456789")); 
    
    // set up socket
    let socket = UdpSocket::bind("0.0.0.0:33333").expect("failed to bind host socket");
    socket.set_read_timeout(Some(Duration::new(5, 0))).expect("failed to set read timeout");
    
    // send ping
    ping_request.send_to(socket.try_clone().expect("fail"), bootstrap_servers[0].clone());

    // recv ping response
    let mut recv_buff = vec![0; 8092];
    println!("Awaiting responses...");   // self.recv_buff is a [u8; 8092]
    while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
        println!("{} bytes response from {:?}", n, addr);
        println!("{}", String::from_utf8_lossy(&recv_buff[0..n]));
        return
    }
}
