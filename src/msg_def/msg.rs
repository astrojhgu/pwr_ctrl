#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_pass_by_value)]
use super::msgcont::{self, Decode};
use std;
use msg_def::PwrMsg::PwrCtrl;

#[derive(Debug, Clone)]
pub enum PwrMsg {
    PwrCtrl {
        content: msgcont::PwrCtrl,
    },
    PwrStat {
        content: msgcont::PwrStat,
    },
    Ack {
        content: msgcont::Ack,
    },
}

impl PwrMsg {
    pub fn pwr_ctrl(d:u8)->PwrMsg{
        let d=d&0xf;
        PwrMsg::PwrCtrl {content:msgcont::PwrCtrl_([d as u32])}
    }

    pub fn header() -> u32 {
        0xaaaa_aaaa
    }

    pub fn tailer() -> u32 {
        0xaaaa_aaaa
    }

    pub fn type_code(&self) -> u32 {
        match *self {
            PwrMsg::PwrCtrl { .. } => 0x5000,
            PwrMsg::PwrStat { .. } => 0x5100,
            PwrMsg::Ack { .. } => 0x5D00,
        }
    }

    pub fn type_name(&self) -> &str {
        match *self {
            PwrMsg::PwrCtrl { .. } => "PWRCTRL",
            PwrMsg::PwrStat { .. } => "PWRSTAT",
            PwrMsg::Ack { .. } => "ACK",
        }
    }

    pub fn get_content_pulp(&self) -> Option<&[u32]> {
        match *self {
            PwrMsg::PwrCtrl { ref content, .. } => Some(&content.0),
            PwrMsg::PwrStat { ref content, .. } => Some(&content.0),
            PwrMsg::Ack { ref content, .. } => Some(&content.0),
        }
    }

    pub fn get_payload_word_vec(&self) -> Option<Vec<u32>> {
        None
    }

    pub fn to_word_vec(&self) -> Vec<u32> {
        let mut result = vec![Self::header(), self.type_code()];
        if let Some(content_pulp) = self.get_content_pulp() {
            result.extend_from_slice(content_pulp);
        }
        if let Some(mut payload) = self.get_payload_word_vec() {
            result.append(&mut payload);
        }
        result.push(Self::tailer());
        result
    }

    pub fn from_word_vec(data: Vec<u32>) -> Option<PwrMsg> {
        assert!(data[0] == Self::header());
        assert!(data[data.len() - 1] == Self::tailer());
        match data[1] {
            0x5000 => msgcont::PwrCtrl::decode(&data[2..]).map(|x| PwrMsg::PwrCtrl { content: x }),
            0x5100 => msgcont::PwrStat::decode(&data[2..]).map(|x| PwrMsg::PwrStat { content: x }),
            0x5D00 => msgcont::Ack::decode(&data[2..]).map(|x| PwrMsg::Ack { content: x }),
            _ => None,
        }
    }

    pub fn to_byte_vec(&self) -> Vec<u8> {
        let word_slice = self.to_word_vec().into_boxed_slice();
        let cap = word_slice.len() * 4;
        unsafe { Vec::from_raw_parts(Box::into_raw(word_slice) as *mut u8, cap, cap) }
    }

    pub fn from_byte_vec(data: Vec<u8>) -> Option<PwrMsg> {
        let word_cap = data.len() / 4;
        if word_cap * 4 != data.len() {
            return None;
        }
        assert!(data.len() % 4 == 0);
        let temp_vec = vec![0_u32; data.len() / 4];
        let ptr_temp_vec =
            unsafe { std::slice::from_raw_parts_mut(temp_vec.as_ptr() as *mut u8, data.len()) };
        ptr_temp_vec
            .iter_mut()
            .zip(data.into_iter())
            .for_each(|(a, b)| *a = b);

        Self::from_word_vec(temp_vec)
    }

    pub fn display_as_words(&self) {
        let words = self.to_word_vec();
        words.iter().for_each(|&x| {
            println!("{:0>8x}", x);
        })
    }
}
