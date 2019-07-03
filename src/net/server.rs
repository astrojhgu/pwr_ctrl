use super::super::msg_def::msg::PwrMsg;

use super::codec::MsgDecoder;
use std;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
//use tokio::codec::Decoder;
use tokio::net::{UdpFramed, UdpSocket as TUdpSocket};
use tokio::prelude::Future;
use tokio::prelude::Stream;

pub struct PwrServer {
    socket: UdpSocket,
    handlers: Vec<Box<dyn FnMut(&PwrMsg, std::net::SocketAddr) -> () + Send + Sync>>,
}

impl PwrServer {
    pub fn register_handler(
        &mut self,
        h: Box<dyn FnMut(&PwrMsg, std::net::SocketAddr) -> () + Send + Sync>,
    ) {
        self.handlers.push(h);
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = vec![0_u8; 65536];
            let (s, addr) = self.socket.recv_from(&mut buf[..]).unwrap();
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //eprintln!("{}", buf.len());

            if let Some(ref msg) = PwrMsg::from_byte_vec(buf) {
                for h in &mut self.handlers {
                    h(&msg, addr);
                }
            }
        }
    }

    pub fn wait_for(&mut self, dt: Option<Duration>) -> Option<PwrMsg> {
        let mut buf = vec![0_u8; 65536];
        self.socket
            .set_read_timeout(dt)
            .expect("set timeout failed");
        if let Ok((s, _addr)) = self.socket.recv_from(&mut buf[..]) {
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //println!("{}", buf.len());
            PwrMsg::from_byte_vec(buf)
        } else {
            None
        }
    }

    pub fn new(addr: SocketAddr) -> Self {
        PwrServer {
            socket: UdpSocket::bind(&addr)
                .unwrap_or_else(|_| panic!("bind to addr {} failed", addr)),
            handlers: Vec::new(),
        }
    }
}

pub fn create_async_server(
    addr: SocketAddr,
    handler: impl FnMut((PwrMsg, SocketAddr)) -> Result<(), <UdpFramed<MsgDecoder> as Stream>::Error>,
) -> impl Future<Item = (), Error = ()> {
    println!("port={}", addr.port());
    UdpFramed::new(
        TUdpSocket::bind(&addr).expect("bind failed3"),
        MsgDecoder {},
    )
    //.for_each(|(msg, _socket)| { Ok(())})
    .for_each(handler)
    .map_err(|_err| {})
}
