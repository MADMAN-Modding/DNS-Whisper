#[tokio::test]
async fn test_socket() {
    use tokio::net::UdpSocket;

    let socket = UdpSocket::bind("127.0.0.1:5253").await.unwrap();

    let write_msg = "test";

    let write_size = socket.send_to(write_msg.as_bytes(), "127.0.0.1:5253").await.unwrap();

    println!("{}", write_size);

    let mut buf = [0; 10];

    let read = socket.recv_from(&mut buf).await.unwrap();

    assert!(write_size == read.0); 

    let read_msg = String::from_utf8(buf[..read.0].to_vec()).unwrap();

    assert!(String::from(write_msg) == read_msg);
}