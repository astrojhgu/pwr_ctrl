#![allow(unused_imports)]

extern crate pwr_ctrl;
use std::io::Read;
use std::str;
//use pwr_ctrl::msgcont::Daq;
use std::fs::File;

use std::env;

use std::net::UdpSocket;

use pwr_ctrl::net::client::send_msg;
use pwr_ctrl::msg_def::msg::PwrMsg;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <stat> <addr:port> <monitor port>", args[0]);
        return;
    }

    let d=env::args().nth(1).unwrap().parse::<u8>().unwrap();

    let addr = env::args().nth(2).expect("Invalid addr");
    let monitor_port=env::args().nth(3).unwrap().parse().unwrap();

    match send_msg(addr.clone(), PwrMsg::pwr_ctrl(d), Some(monitor_port)) {
                Ok(..) => println!("Corresponding Ack received"),
                Err(x) => println!("{:?}", x),
            };

}
