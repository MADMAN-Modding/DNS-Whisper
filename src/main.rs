use std::env;

use dns_whisper::server::{self, util::build_query};
use tokio::{io, net::UdpSocket, time::{self}};

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.get(1).unwrap_or(&String::new()) == "--server" {
        server::start::spawn_dns_server(1234, true).await;
    } else {
        let query = build_query("example.com", 1);

        let sock = UdpSocket::bind("0.0.0.0:8080").await?;

        server::start::spawn_dns_server(1234, false).await;

        sock.connect("127.0.0.1:1234").await?;

        let len = sock.send(&query).await?;
        println!("Send Size: {len}");

        time::sleep(time::Duration::from_secs(1)).await;
    }

    println!("Hello, world!");

    Ok(())
}
