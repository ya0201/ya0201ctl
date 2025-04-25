extern crate ipnet;
use ipnet::IpNet;
use std::str::FromStr;
pub fn run(cidr: String) {
    // println!("Subnet info: {}", cidr);
    let net = IpNet::from_str(&cidr).unwrap();
    println!("{} hostmask = {}", net, net.hostmask());
    println!("{} netmask = {}", net, net.netmask());
}
