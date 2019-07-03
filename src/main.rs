#![allow(unused_imports)]

extern crate pwr_ctrl;
extern crate tokio;
//use std::env;
use pwr_ctrl::net::codec::MsgDecoder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::codec::Decoder;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::Future;
use tokio::prelude::Stream;

fn main() {
    let server1 = pwr_ctrl::net::server::create_async_server(
        SocketAddr::from_str("0.0.0.0:1234").unwrap(),
        |(msg, _socket)| {
            println!("{:?}", msg);
            Ok(())
        },
    );
    let server2 = pwr_ctrl::net::server::create_async_server(
        SocketAddr::from_str("0.0.0.0:1235").unwrap(),
        |(msg, _socket)| {
            println!("{:?}", msg);
            Ok(())
        },
    );

    let _j = std::thread::spawn(|| {
        tokio::run(server1.join(server2).map(|_| {}));
    });
    println!("a");
    //tokio::run(server1);
}
