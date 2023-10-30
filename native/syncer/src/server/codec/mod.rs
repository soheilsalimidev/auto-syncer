use std::mem::size_of;
mod codecer;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
enum ParserState {
    Length,
    Data,
}

pub struct Codec {
    state: ParserState,
    message_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Massage {
    Accepted(usize),
    Message(String),
    Disconnected(),
}

const HEADER_SIZE: usize = size_of::<u32>();
const DEFAULT_MAX_RECV_MESSAGE_SIZE: usize = 4 * 1024 * 1024;
const DEFAULT_MAX_SEND_MESSAGE_SIZE: usize = usize::MAX;
