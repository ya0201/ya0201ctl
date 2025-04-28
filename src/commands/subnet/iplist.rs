use std::io::{stdout, BufWriter, Write};
extern crate ipnet;
use ipnet::IpNet;
use std::str::FromStr;

pub fn run(cidr: String) {
    let net = IpNet::from_str(&cidr).unwrap();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    writeln!(out, "{}", net.network()).unwrap();
    for ip in net.hosts() {
        writeln!(out, "{}", ip).unwrap();
    }
    writeln!(out, "{}", net.broadcast()).unwrap();
}
