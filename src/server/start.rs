use tokio::io;

use crate::{BUF_SIZE, parse, server::util};

pub async fn start_dns(port: u16) -> io::Result<()> {
    let server = util::start_udp(port).await?;

    loop {
        let mut buf = [0u8; BUF_SIZE];
        let read_len = server.recv(&mut buf).await?;
        println!("Read Size: {read_len}");

        parse::util::process_incoming(&buf);
    }
}

pub async fn spawn_dns_server(port: u16, wait: bool) {
    let task = tokio::task::spawn(async move {
        let res= start_dns(port).await;

        if res.is_err() {
            return;
        }
    });

    if wait {
        let _ = task.await;
    }
}