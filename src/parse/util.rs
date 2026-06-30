use std::{
    io::{self, BufRead}, net::{IpAddr, Ipv6Addr},
};

use hickory_proto::{
    op::Message,
    rr::{Name, RData, Record, rdata::AAAA},
};
use rand::distr::{Alphanumeric, SampleString};

use crate::{parse, server::util};

pub fn parse_address_from_query(msg: &Message) -> Name {
    if msg.queries.len() == 0 {
        return Name::new();
    }
    let name = msg.queries[0].name.clone();

    return name;
}

pub fn parse_reply_from_response(msg: &Message) {
    let response = &msg.answers[0];

    let addr: IpAddr = response.data.ip_addr().unwrap();

    let bytes = match addr {
        IpAddr::V4(v4) => v4.octets().to_vec(),
        IpAddr::V6(v6) => v6.octets().to_vec(),
    };
    let response_text = String::from_utf8_lossy(&bytes).into_owned();

    let result = response_text.find("\n");

    let response_text = if result.is_some() {
        response_text.split_at(result.unwrap()).0
    } else {
        &response_text
    };

    println!("Reply: {response_text}")
}

pub fn process_incoming(buf: &[u8]) {
    let msg = util::parse_query(&buf);

    println!("Message: {msg}");

    let name = parse::util::parse_address_from_query(&msg);

    println!("Name: {name}");

    let mut reply: String = String::new();

    // Get user input
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        reply = format!("{reply}{}", line.unwrap());
        break;
    }

    if reply.len() <= 14 {
        reply += "\n";

        let string = Alphanumeric.sample_string(&mut rand::rng(), 16 - reply.len());
        println!("{string}");

        reply += &string;

        drop(string);
    }

    let response = parse::util::build_response(&msg, &reply);

    println!("Response: {}", response);

    parse_reply_from_response(&response);
}

pub fn build_response(msg: &Message, reply: &str) -> Message {
    let name = parse_address_from_query(msg);

    let bytes = reply.as_bytes();

    let mut octets = [0u8; 16];
    let len = bytes.len().min(16);
    octets[..len].copy_from_slice(&bytes[..len]);

    let ipv6_addr = Ipv6Addr::from_octets(octets);

    let rdata = RData::AAAA(AAAA::from(ipv6_addr));

    let record = Record::from_rdata(name, 200, rdata);

    let mut response = msg.clone().into_response();
    response.add_answer(record);

    response
}
