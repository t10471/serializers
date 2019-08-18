// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod p2p {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MessageType {
  Handshake = 0,
  SendTransaction = 1,
  SendBlock = 2,
  Bye = 3,

}

const ENUM_MIN_MESSAGE_TYPE: i32 = 0;
const ENUM_MAX_MESSAGE_TYPE: i32 = 3;

impl<'a> flatbuffers::Follow<'a> for MessageType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for MessageType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i32::to_le(self as i32);
    let p = &n as *const i32 as *const MessageType;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i32::from_le(self as i32);
    let p = &n as *const i32 as *const MessageType;
    unsafe { *p }
  }
}

impl flatbuffers::Push for MessageType {
    type Output = MessageType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<MessageType>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_MESSAGE_TYPE:[MessageType; 4] = [
  MessageType::Handshake,
  MessageType::SendTransaction,
  MessageType::SendBlock,
  MessageType::Bye
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_MESSAGE_TYPE:[&'static str; 4] = [
    "Handshake",
    "SendTransaction",
    "SendBlock",
    "Bye"
];

pub fn enum_name_message_type(e: MessageType) -> &'static str {
  let index = e as i32;
  ENUM_NAMES_MESSAGE_TYPE[index as usize]
}

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Status {
  OK = 0,
  NG = 1,

}

const ENUM_MIN_STATUS: i32 = 0;
const ENUM_MAX_STATUS: i32 = 1;

impl<'a> flatbuffers::Follow<'a> for Status {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Status {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i32::to_le(self as i32);
    let p = &n as *const i32 as *const Status;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i32::from_le(self as i32);
    let p = &n as *const i32 as *const Status;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Status {
    type Output = Status;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Status>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_STATUS:[Status; 2] = [
  Status::OK,
  Status::NG
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_STATUS:[&'static str; 2] = [
    "OK",
    "NG"
];

pub fn enum_name_status(e: Status) -> &'static str {
  let index = e as i32;
  ENUM_NAMES_STATUS[index as usize]
}

pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      if let Some(x) = args.payload { builder.add_payload(x); }
      builder.add_status(args.status);
      builder.add_type_(args.type_);
      builder.finish()
    }

    pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
    pub const VT_STATUS: flatbuffers::VOffsetT = 6;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn type_(&self) -> MessageType {
    self._tab.get::<MessageType>(Message::VT_TYPE_, Some(MessageType::Handshake)).unwrap()
  }
  #[inline]
  pub fn status(&self) -> Status {
    self._tab.get::<Status>(Message::VT_STATUS, Some(Status::OK)).unwrap()
  }
  #[inline]
  pub fn payload(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Message::VT_PAYLOAD, None).map(|v| v.safe_slice())
  }
}

pub struct MessageArgs<'a> {
    pub type_: MessageType,
    pub status: Status,
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            type_: MessageType::Handshake,
            status: Status::OK,
            payload: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_type_(&mut self, type_: MessageType) {
    self.fbb_.push_slot::<MessageType>(Message::VT_TYPE_, type_, MessageType::Handshake);
  }
  #[inline]
  pub fn add_status(&mut self, status: Status) {
    self.fbb_.push_slot::<Status>(Message::VT_STATUS, status, Status::OK);
  }
  #[inline]
  pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_PAYLOAD, payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_root::<Message<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_size_prefixed_root::<Message<'a>>(buf)
}

#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod P2P

