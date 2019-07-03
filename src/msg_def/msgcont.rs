#![allow(clippy::identity_op)]
//use bitfield::*;

pub type PwrCtrl = PwrCtrl_<[u32; 1]>;
pub type PwrStat = PwrStat_<[u32; 1]>;
pub type Ack = Ack_<[u32; 2]>;

pub trait Decode
where
    Self: Sized,
{
    fn decode(_: &[u32]) -> Option<Self>;
}

bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct PwrCtrl_([u32]);
    impl Debug;
    u32;
    pub u8, state, set_stat: 0, 0;//1
}

impl Decode for PwrCtrl {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 1];
            result.copy_from_slice(&data[..]);
            Some(PwrCtrl_(result))

    }
}

bitfield! {

    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct PwrStat_([u32]);
    impl Debug;
    u32;
    pub u8, stat, set_stat:0,0;
}

impl Decode for PwrStat {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 1];
        result.copy_from_slice(&data[..]);
        Some(PwrStat_(result))
    }
}


bitfield! {
    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct Ack_([u32]);
    impl Debug;
    u32;
    pub u32,ip, set_ip:31, 0;
    pub u16, msg_ack, set_msg_ack:32+15, 32+0;
}

impl Decode for Ack {
    fn decode(data: &[u32]) -> Option<Self> {
        let mut result = [0_u32; 2];
        if data.len() < 2 {
            None
        } else {
            result.copy_from_slice(&data[..2]);
            Some(Ack_(result))
        }
    }
}
