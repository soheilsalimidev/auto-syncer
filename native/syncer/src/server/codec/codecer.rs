use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{Decoder, Encoder};

use super::{Massage, HEADER_SIZE, Codec, ParserState};


impl Decoder for Codec {
    type Item = Massage;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Massage>, std::io::Error> {
        self.parse_length(src)
    }
}

impl Encoder<Massage> for Codec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Massage, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let mut bytes = Vec::new();
        item.serialize(&mut Serializer::new(&mut bytes))
            .map_err(|_| std::io::ErrorKind::InvalidData)?;
        dst.put_u32(bytes.len() as u32);
        dst.put(&bytes[..]);
        Ok(())
    }
}

impl Codec {
    pub fn new() -> Self {
        Self {
            state: ParserState::Length,
            message_size: 0,
        }
    }

    fn parse_length(&mut self, buf: &mut BytesMut) -> Result<Option<Massage>, std::io::Error> {
        // Try to find the current length.
        if self.state == ParserState::Length {
            if buf.len() < HEADER_SIZE {
                return Ok(None);
            }
            self.message_size = buf.get_u32() as _;
            buf.reserve(self.message_size);
            self.state = ParserState::Data;
        }

        if self.state == ParserState::Data {
            return self.parse_data(buf);
        }

        Ok(None)
    }

    fn parse_data(&mut self, buf: &mut BytesMut) -> Result<Option<Massage>, std::io::Error> {
        if buf.len() >= self.message_size {
            let data = Massage::deserialize(&mut Deserializer::new(
                &*buf.split_to(self.message_size),
            ))
            .map_err(|f| std::io::Error::new(std::io::ErrorKind::InvalidData, f.to_string()))?;
            self.state = ParserState::Length;
            self.message_size = 0;
            return Ok(Some(data));
        }

        Ok(None)
    }
}

