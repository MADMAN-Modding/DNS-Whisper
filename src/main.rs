use std::env;

use dns_whisper::server::util::build_query;
use tokio::net::UdpSocket;

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.get(1).unwrap_or(&String::new()) == "--server" {

    } else {
        let query = build_query("example.com", 1);

        
    }

    println!("Hello, world!");
}

