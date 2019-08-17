extern crate bytes;
extern crate protobuf;
mod p2p;

use bytes::Bytes;

use protobuf::*;

fn main() {
    let m = p2p::Message::new();
    let buf = Bytes::from(m.write_to_bytes().unwrap());
    println!("Serialize: {:?}", buf);
    let msg = parse_from_bytes::<p2p::Message>(&buf).unwrap();
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.get_field_type());

    let mut m = p2p::Message::new();
    m.set_field_type(p2p::Message_MessageType::Handshake);
    let buf = Bytes::from(m.write_to_bytes().unwrap());
    println!("Serialize: {:?}", buf);
    let msg = parse_from_bytes::<p2p::Message>(&buf).unwrap();
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.get_field_type());

    let mut m = p2p::Message::new();
    m.set_field_type(p2p::Message_MessageType::Handshake);
    m.set_status(p2p::Message_Status::NG);
    let buf = Bytes::from(m.write_to_bytes().unwrap());
    println!("Serialize: {:?}", buf);
    let msg = parse_from_bytes::<p2p::Message>(&buf).unwrap();
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.get_field_type());

    let mut m = p2p::Message::new();
    m.set_field_type(p2p::Message_MessageType::Bye);
    let buf = Bytes::from(m.write_to_bytes().unwrap());
    println!("Serialize: {:?}", buf);
    let msg = parse_from_bytes::<p2p::Message>(&buf).unwrap();
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.get_field_type());
}
