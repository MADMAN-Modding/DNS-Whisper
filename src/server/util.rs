use tokio::net::UdpSocket;

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