#[tokio::test]  
async fn test_socket() {
    use crate::server::util::start_udp;

    // Port
    let port: u16 = 4242;

    // Start UDP Server
    let socket = start_udp(port).await.unwrap();

    // Message to write
    let write_msg = "test";

    // Bytes written
    let write_size = socket.send_to(write_msg.as_bytes(), format!("127.0.0.1:{port}")).await;

    // Read successful
    assert!(write_size.is_ok());

    // Buffer to read data
    let mut buf = [0; 10];

    // Read data
    let read = socket.recv_from(&mut buf).await.unwrap();

    // Written data is the same length as read data
    assert!(write_size.unwrap() == read.0); 

    // Read message in utf8
    let read_msg = String::from_utf8(buf[..read.0].to_vec()).unwrap();

    // Messages are equal
    assert!(String::from(write_msg) == read_msg);
}

