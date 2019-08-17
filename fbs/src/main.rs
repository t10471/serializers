extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./p2p_generated.rs"]
mod p2p_generated;
pub use p2p_generated::p2p::{get_root_as_message, Message, MessageArgs, MessageType, Status};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(0);
    let m = Message::create(&mut builder, &Default::default());
    builder.finish(m, None);
    let buf = builder.finished_data();
    println!("Serialize: {:?}", buf);
    let msg = get_root_as_message(buf);
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.type_());

    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(0);
    let m = Message::create(
        &mut builder,
        &MessageArgs {
            type_: MessageType::Handshake,
            ..Default::default()
        },
    );
    builder.finish(m, None);
    let buf = builder.finished_data();
    println!("Serialize: {:?}", buf);
    let msg = get_root_as_message(buf);
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.type_());

    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(0);
    let m = Message::create(
        &mut builder,
        &MessageArgs {
            type_: MessageType::Handshake,
            status: Status::NG,
            ..Default::default()
        },
    );
    builder.finish(m, None);
    let buf = builder.finished_data();
    println!("Serialize: {:?}", buf);
    let msg = get_root_as_message(buf);
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.type_());

    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(0);
    let m = Message::create(
        &mut builder,
        &MessageArgs {
            type_: MessageType::Bye,
            ..Default::default()
        },
    );
    builder.finish(m, None);
    let buf = builder.finished_data();
    println!("Serialize: {:?}", buf);
    let msg = get_root_as_message(buf);
    println!("Deserialize: {:?}", msg);
    println!("Type: {:?}", msg.type_());
}
