use hickory_proto::{op::Message, rr::Name};

use crate::{parse, server::util};

pub fn parse_address_from_query(msg: Message) -> Name {
    if msg.queries.len() == 0 {
        return Name::new();
    }
    let name = msg.queries[0].name.clone();
    drop(msg);

    return name;
}

pub fn process_incoming(buf: &[u8]) {
        let msg = util::parse_query(&buf);

        println!("Message: {msg}");

        let name = parse::util::parse_address_from_query(msg);

        println!("Name: {name}");
}

pub fn build_response(msg: Message) {
    // let rdata = Record::record_type(rdata::AAAA::new(a, b, c, d, e, f, g, h));

    // let record = Record::from_rdata(name, 1234, rdata);

    // msg.add_answer(record);
}