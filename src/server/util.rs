use tokio::net::UdpSocket;
use hickory_proto::op::{Message, MessageType, OpCode, Query};
use hickory_proto::rr::{Name, RecordType, DNSClass};
use hickory_proto::serialize::binary::{BinDecodable, BinDecoder, BinEncodable, BinEncoder};
use std::str::FromStr;
/// Starts UDP Server on the specified port
/// 
/// # Example
/// ```
/// #[tokio::main]
/// async fn main() {
///     use dns_whisper::server::util::start_udp;
/// 
///     let server = start_udp(4242).await;
/// 
///     // Error handling logic here
/// }
/// ```
pub fn start_udp(port: u16) -> impl Future<Output = Result<UdpSocket, std::io::Error>> {
    let addr = format!("127.0.0.1:{port}");
    
    return UdpSocket::bind(addr);
}

/// Read utf8 String from a buffer to a fixed length
/// 
/// # Example
/// ```
/// #[tokio::main]
/// async fn main() {
///     use dns_whisper::server::util::read_string_from_buf;
/// 
///     let buf: &[u8] = &[12,42,41,2];
/// 
///     let length: usize = 4;
/// 
///     let string: String = read_string_from_buf(buf, length).await;
/// }
/// ```
pub async fn read_string_from_buf(buf: &[u8], length: usize) -> String{
    String::from_utf8(buf[..length].to_vec()).unwrap()
}

/// Build a query with a set ID
/// 
/// # Example
/// ```
/// use dns_whisper::server::util::build_query;
/// 
/// let query_bytes: Vec<u8> = build_query("foo.example.com", 1234);
///     
/// ```
pub fn build_query(domain: &str, id: u16) -> Vec<u8> {
    let mut message = Message::new(id, MessageType::Query, OpCode::Query);

    let name = Name::from_str(domain).unwrap();
    let mut query = Query::new();
    query.set_name(name)
         .set_query_type(RecordType::AAAA)
         .set_query_class(DNSClass::IN);

    message.add_query(query);

    let mut buf = Vec::new();
    let mut encoder = BinEncoder::new(&mut buf);
    message.emit(&mut encoder).unwrap();
    buf
}

/// Parse incoming bytes back into a Message
/// 
/// # Example
/// ```
/// use hickory_proto::op::Message;
/// use dns_whisper::server::util::build_query;
/// use dns_whisper::server::util::parse_query;
/// 
/// let query_bytes: Vec<u8> = build_query("foo.example.com", 1234);
/// 
/// let message: Message = parse_query(&query_bytes);
/// ```
pub fn parse_query(bytes: &[u8]) -> Message {
    let mut decoder = BinDecoder::new(bytes);
    Message::read(&mut decoder).unwrap()
}