pub mod queries;

#[macro_use] extern crate clap;

use clap::App;
use ansi_term::Colour;
use ansi_term::Style;
use queries::PingRequest;
use queries::PingRequestPayload;
use std::net::UdpSocket;
use std::time::Duration;
use std::net::ToSocketAddrs;
use std::str;



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

    let ping_request = PingRequest {
        t: String::from("b7"),
        y: String::from("q"),
        q: String::from("ping"),
        a: PingRequestPayload {
            id: String::from("abcdefghij0123456789")
        }
    };
    let string = serde_bencoded::to_vec(&ping_request).unwrap();

    println!("{}", std::str::from_utf8(&string).unwrap());

    let socket = UdpSocket::bind("0.0.0.0:33333").expect("failed to bind host socket");
    socket.set_read_timeout(Some(Duration::new(5, 0)));
    println!("Broadcast: {:?}", socket.broadcast());
    println!("Timeout: {:?}", socket.read_timeout());

    match socket.send_to(&string, bootstrap_servers[0].clone()) {
        Ok(n) => {
            if n != string.len() {
                return
            }
            else {
                // Do nothing because we sent the number of bytes we expected to send
            }
        },
        Err(e) => {
            println!("{}", e);
            return
        }
    }

    let mut recv_buff = vec![0; 8092];

    println!("Awaiting responses...");   // self.recv_buff is a [u8; 8092]
    while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
        println!("{} bytes response from {:?}", n, addr);
        println!("{}", String::from_utf8_lossy(&recv_buff));
        // Remaining code not directly relevant to the question
    }
}
