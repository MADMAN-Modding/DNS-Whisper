#[tokio::test]  
async fn socket() {
    use crate::server::util::start_udp;
    use crate::server::util::read_string_from_buf;

    const PORT: u16 = 4242;

    // Start UDP Server
    let socket = start_udp(PORT).await.unwrap();

    // Message to write
    let write_msg = "test";

    // Bytes written
    let write_size = socket.send_to(write_msg.as_bytes(), format!("127.0.0.1:{PORT}")).await;

    // Read successful
    assert!(write_size.is_ok());

    // Buffer to read data
    let mut buf = [0; 10];

    // Read data
    let read = socket.recv_from(&mut buf).await.unwrap();

    // Written data is the same length as read data
    assert!(write_size.unwrap() == read.0); 

    // Read message in utf8
    let read_msg = read_string_from_buf(&buf, read.0).await;

    // Messages are equal
    assert!(String::from(write_msg) == read_msg);
}

#[test]
fn test_build_query_valid_domain() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;


    let bytes = build_query("foo.bar.com",1234);
    assert!(!bytes.is_empty());

    // should round-trip parse without panicking
    let msg = parse_query(&bytes);
    assert_eq!(msg.queries.len(), 1);
}

#[test]
fn test_build_query_domain_preserved() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;

    let bytes = build_query("foo.bar.com", 1234);
    let msg = parse_query(&bytes);
    let name = msg.queries[0].name().to_string();
    assert_eq!(name, "foo.bar.com.");  // note trailing dot, that's normal
}

#[test]
fn test_build_query_record_type() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;
    use hickory_proto::rr::RecordType;

    let bytes = build_query("foo.bar.com", 1234);
    let msg = parse_query(&bytes);
    assert_eq!(msg.queries[0].query_type(), RecordType::TXT);
}

#[test]
fn test_build_query_message_type() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;
    use hickory_proto::op::MessageType;
    

    let bytes = build_query("foo.bar.com", 1234);
    let msg = parse_query(&bytes);
    assert_eq!(msg.message_type, MessageType::Query);
}

#[test]
fn test_parse_query_id_preserved() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;

    let bytes = build_query("foo.bar.com", 1234);
    let msg = parse_query(&bytes);
    assert_eq!(msg.id, 1234);
}

#[test]
fn test_build_query_subdomain() {
    use crate::server::util::build_query;
    use crate::server::util::parse_query;

    // simulates what encoded payload queries will look like
    let bytes = build_query("AAEBCD.tunnel.yourdomain.com",1234);
    let msg = parse_query(&bytes);
    let name = msg.queries[0].name().to_string();
    assert_eq!(name, "AAEBCD.tunnel.yourdomain.com.".to_ascii_lowercase());
}

#[test]
#[should_panic]
fn test_parse_query_garbage_bytes() {
    use crate::server::util::parse_query;

    parse_query(&[0xFF, 0x00, 0xAB]);
}
