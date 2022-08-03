//https://github.com/evilsocket/medusa
//https://users.rust-lang.org/t/function-pointers-in-a-hashmap/6530

#![allow(clippy::print_literal)]

extern crate procfs;
use nettop_rs;
use procfs::process::{FDTarget, Stat};
use std::collections::HashMap;

use crate::netdata::tcp::sockets::test_fn;
mod netdata;
mod lib;


fn main() {
    
    let mut o = lib::Observer::new();

    let x = lib::NetFactory::new_net_io(&lib::NetType::TCP4);
    let y = lib::NetFactory::new_net_io(&lib::NetType::TCP4);

    o.add_data_stream(x);
    o.add_data_stream(y);
    o.get_data();
    
}
