use super::super::msg_def::msg::PwrMsg;
use bytes::BytesMut;
use tokio::codec::Decoder;

use std;

pub struct MsgDecoder {}

impl Decoder for MsgDecoder {
    type Item = PwrMsg;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&src.take());
        Ok(PwrMsg::from_byte_vec(buff))
    }
}
