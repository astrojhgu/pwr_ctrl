#![allow(unused_imports)]

extern crate chrono;
extern crate pwr_ctrl;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

use chrono::offset::Utc;

use pwr_ctrl::msg_def::msgcont::Ack_;
use pwr_ctrl::msg_def::PwrMsg;
use pwr_ctrl::net::client::send_msg;
use pwr_ctrl::net::server::PwrServer;

//deprecated
//use pwr_ctrl::io::txt;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <addr:port> <monitor port>",
            args[0]
        );
        return;
    }

    let monitor_port: u16 = args[2].parse().expect("invalid monitor port");

    let mut server = PwrServer::new(args[1].parse().expect("invalid port"));
    server.register_handler(Box::new(|a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    server.run();
}
